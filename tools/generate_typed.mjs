#!/usr/bin/env node
// Schema interpretation is delegated to Typify and jsonschema. This adapter only
// translates OAS 3.0 vocabulary and emits operation bindings to the shared transport.
import { readFile, writeFile, mkdir } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const args = process.argv.slice(2);
const outputIndex = args.indexOf('--out-dir');
if (outputIndex !== -1 && !args[outputIndex + 1]) throw Error('--out-dir requires a directory');
const outputRoot = path.resolve(outputIndex === -1 ? '.' : args[outputIndex + 1]);
if (outputIndex !== -1) args.splice(outputIndex, 2);
if (args.length > 1 || args.some(arg => arg.startsWith('--'))) throw Error('Usage: node tools/generate_typed.mjs [bundle.json] [--out-dir directory]');
const bundlePath = args[0] ?? (process.env.KALSHICORE_ROOT && path.join(process.env.KALSHICORE_ROOT, 'contracts/current-official.json'));
if (!bundlePath) throw Error('Pass a contract bundle or set KALSHICORE_ROOT.');
const bundle = JSON.parse(await readFile(path.resolve(bundlePath), 'utf8'));
const generatorManifest = fileURLToPath(new URL('./typed-generator/Cargo.toml', import.meta.url));
await mkdir(outputRoot, {recursive:true});
process.chdir(outputRoot);
await mkdir('tests', {recursive:true});
await mkdir('tools', {recursive:true});
const source = bundle.sources.openapi.document;
const resolve = obj => obj?.$ref ? resolve(obj.$ref.slice(2).split('/').reduce((v,k) => v[k.replaceAll('~1','/').replaceAll('~0','~')], source)) : obj;
const compat = [];
function normalize(value, pointer) {
  if (Array.isArray(value)) return value.map((v,i) => normalize(v, `${pointer}/${i}`));
  if (!value || typeof value !== 'object') return value;
  const result = {};
  for (const [key, child] of Object.entries(value)) {
    if (key === 'nullable') continue;
    result[key] = key === '$ref' ? child.replace('#/components/schemas/', '#/definitions/') : normalize(child, `${pointer}/${key}`);
  }
  if (value.nullable) {
    if ((value.enum && !value.enum.includes(null)) || (Object.hasOwn(value,'const') && value.const !== null)) {
      compat.push({pointer, fix:'Nullable does not widen enum/const; null remains forbidden. Invalid default is annotation only and is not injected.'});
      return result;
    }
    compat.push({pointer, fix:'OAS nullable -> JSON Schema anyOf with null; original branch retained'});
    return {anyOf:[result,{type:'null'}]};
  }
  return result;
}
const definitions = Object.fromEntries(Object.entries(source.components.schemas).map(([k,v])=>[k, normalize(v, `#/components/schemas/${k}`)]));
const snake = n => n.replace(/([A-Z]+)([A-Z][a-z])/g,'$1_$2').replace(/([a-z0-9])([A-Z])/g,'$1_$2').toLowerCase();
const pascal = n => snake(n).split('_').map(s=>s[0].toUpperCase()+s.slice(1)).join('');
const rust = s => JSON.stringify(s);
const ops = [];
for (const [path, item] of Object.entries(source.paths)) {
  for (const method of ['get','post','put','patch','delete','head','options']) {
    const op = item[method]; if (!op) continue;
    const id = op.operationId, type = pascal(id), parameters = [...(item.parameters??[]),...(op.parameters??[])].map(resolve);
    if (new Set(parameters.map(p=>p.name)).size !== parameters.length) throw Error(`Duplicate parameters: ${id}`);
    const params = `${type}Parameters`;
    if (parameters.length) definitions[params] = {type:'object',properties:Object.fromEntries(parameters.map(p=>[p.name,normalize(p.schema, `#/paths/${path}/${method}/parameters/${p.name}`)])),required:parameters.filter(p=>p.required).map(p=>p.name),additionalProperties:false};
    const body = resolve(op.requestBody);
    if (body && Object.keys(body.content).some(c=>c!=='application/json')) throw Error(`Unsupported body media type: ${id}`);
    const bodyName = body ? `${type}Body` : null;
    if (body) definitions[bodyName]=normalize(body.content['application/json'].schema, `#/paths/${path}/${method}/requestBody`);
    const responses = Object.entries(op.responses).map(([status,r])=>{
      r=resolve(r);
      if (r.content && Object.keys(r.content).some(c=>c!=='application/json')) throw Error(`Unsupported response media type: ${id}`);
      const schema=r.content?.['application/json']?.schema;
      const name=schema ? `${type}Status${status}` : null;
      if (schema) definitions[name]=normalize(schema, `#/paths/${path}/${method}/responses/${status}`);
      return {status,name,schema};
    });
    const success=responses.filter(r=>/^2/.test(r.status));
    if (success.length!==1) throw Error(`Implement success union for ${id}: ${success.map(r=>r.status)}`);
    const cursor=parameters.find(p=>p.in==='query' && p.name==='cursor');
    const responseSchema=success[0].schema ? resolve(success[0].schema) : null;
    const cursorField = responseSchema?.properties?.cursor ? 'cursor' : responseSchema?.properties?.next_cursor ? 'next_cursor' : null;
    const paginated=!!cursor && !!cursorField;
    const cursorRequired = responseSchema?.required?.includes(cursorField);
    if(cursor && !paginated) throw Error(`Cursor response must be examined: ${id}`);
    ops.push({id,type,method:snake(id),http:method.toUpperCase(),path,parameters,params:parameters.length?params:null,bodyName,bodyRequired:!!body?.required,responses,success:success[0].name,successStatus:Number(success[0].status),unknownSuccess:!success[0].name && success[0].status!=='204',paginated,cursorField,cursorRequired,authenticated:!!(op.security??source.security)?.length,source:`#/paths/${path.replaceAll('~','~0').replaceAll('/','~1')}/${method}`});
  }
}
if (ops.length!==118 || ops.some(o=>!bundle.rest.some(r=>r.id===o.id && r.method===o.http && r.path===o.path))) throw Error('Contract operation mismatch');
const schema={$schema:'http://json-schema.org/draft-07/schema#',definitions};
await mkdir('src/generated',{recursive:true});
await writeFile('src/generated/typed-schema.json',JSON.stringify(schema,null,2)+'\n');
const run=spawnSync('cargo',['run','--locked','--manifest-path',generatorManifest,'--','src/generated/typed-schema.json','src/generated/models.rs'],{stdio:'inherit'});
if(run.status!==0) process.exit(run.status??1);
let code='// Generated by tools/generate_typed.mjs. Do not edit.\nuse crate::{client::{KalshiClient, KalshiRequest}, error::KalshiError, typed::{TypedError, Pager, UnspecifiedResponse}};\nuse crate::generated::models::*;\n';
let probes='// Generated signature probes: every operation is type checked.\n#![allow(dead_code)]\nuse kalshi_client::{KalshiClient, generated::{models::*, typed::*}, typed::{TypedError, UnspecifiedResponse}};\n';
let wire='// Generated wire probes: every named method sends and decodes a schema fixture.\nmod typed_support;\nuse kalshi_client::models::*;\nuse serde_json::Value;\n#[tokio::test]\nasync fn all_118_named_methods_send_and_decode_official_schema_models() {\nlet fixtures: Value = serde_json::from_str(include_str!("typed-fixtures.json")).unwrap();\nlet auth = typed_support::auth();\n';
let errorWire='// Generated documented-status error probes.\nmod typed_support;\nuse kalshi_client::models::*;\nuse serde_json::Value;\n#[tokio::test]\nasync fn all_documented_errors_keep_structured_source_and_typed_payload() {\nlet fixtures: Value = serde_json::from_str(include_str!("typed-fixtures.json")).unwrap();\nlet auth = typed_support::auth();\n';
for(const o of ops){
  const error=`${o.type}Error`;
  code+=`#[derive(Clone)]\npub enum ${error} {\n${o.responses.filter(r=>!/^2/.test(r.status)).map(r=>`    Status${r.status}${r.name?`(${r.name})`:''},`).join('\n')}\n}\n`;
  code+=`impl std::fmt::Debug for ${error} { fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result { f.write_str("${error}([REDACTED])") } }\n`;
  code+=`impl ${error} { fn decode(error: &KalshiError) -> Option<Self> { if let KalshiError::Api(api) = error { match api.status.as_u16() {\n${o.responses.filter(r=>!/^2/.test(r.status)).map(r=>`${r.status} => ${r.name?`crate::typed::decode_model::<${r.name}>(${rust(r.name)}, api.body.clone()).ok().map(Self::Status${r.status})`:`Some(Self::Status${r.status})`},`).join('\n')}\n_ => None, } } else { None } } }\n`;
  const args=[o.params?`parameters: &${o.params}`:null,o.bodyName?`body: ${o.bodyRequired?'&':'Option<&'}${o.bodyName}${o.bodyRequired?'':'>'}`:null].filter(Boolean).join(', ');
  const result=o.success??(o.unknownSuccess?'UnspecifiedResponse':'()');
  code+=`impl KalshiClient {\n pub async fn ${o.method}(&self${args?', '+args:''}) -> Result<${result}, TypedError<${error}>> {\n let context = self.request_context().map_err(|source| TypedError::new(source, None))?;\n context.run_typed(async {\n let ${o.parameters.length || o.bodyName ? 'mut ' : ''}request = KalshiRequest { authenticated: ${o.authenticated}, ..Default::default() };\n`;
  if(o.params) code+=` let parameters = serde_json::to_value(parameters)?;\n crate::typed::validate(${rust(o.params)}, &parameters)?;\n`;
  for(const p of o.parameters){
    const insert=p.in==='path'?`request.path_parameters.insert(${rust(p.name)}.into(), crate::typed::parameter_text(value)?);`:`request.query.push((${rust(p.name)}.into(), crate::typed::query_value(value)?));`;
    code+=` if let Some(value) = parameters.get(${rust(p.name)}) { ${insert} }\n`;
  }
  if(o.bodyName) code+=o.bodyRequired?` let body = serde_json::to_value(body)?; crate::typed::validate(${rust(o.bodyName)}, &body)?; request.body = Some(body);\n`:` if let Some(body) = body { let body = serde_json::to_value(body)?; crate::typed::validate(${rust(o.bodyName)}, &body)?; request.body = Some(body); }\n`;
  code+=o.unknownSuccess ? ` self.request_unspecified_in_context(${rust(o.id)}, request, &context).await\n` : ` let response = self.request_in_context(${rust(o.id)}, request, &context).await?;\n ${o.success?`crate::typed::decode_model(${rust(o.success)}, response)`:'if response.is_null() { Ok(()) } else { Err(crate::typed::contract_error("unexpected body on an empty response")) }'}\n`;
  code+=` }, ${error}::decode).await\n }\n`;
  if(o.paginated){
    code+=` pub fn ${o.method}_pages(&self, parameters: ${o.params}) -> Pager<'_, ${o.params}, ${result}, ${error}> {\n Pager::new(self, parameters, |client, parameters| Box::pin(async move { client.${o.method}(&parameters).await }), |parameters, cursor| parameters.cursor = Some(cursor), |response| ${o.cursorRequired?'Some(':''}response.${o.cursorField}.clone()${o.cursorRequired?')':''}.filter(|s| !s.is_empty()))\n }\n`;
  }
  code+='}\n';
  probes+=`async fn probe_${o.method}(client: &KalshiClient${args?', '+args:''}) { let _: Result<${result}, TypedError<${error}>> = client.${o.method}(${[o.params?'parameters':null,o.bodyName?'body':null].filter(Boolean).join(', ')}).await; }\n`;
  wire+=`{ let ${o.params||o.bodyName||o.success?'fixture':'_fixture'} = &fixtures[${rust(o.id)}];\n`;
  if(o.params) wire+=`let parameters: ${o.params} = serde_json::from_value(fixture["parameters"].clone()).expect(${rust(o.id+' parameters')});\n`;
  if(o.bodyName) wire+=`let body: ${o.bodyName} = serde_json::from_value(fixture["body"].clone()).expect(${rust(o.id+' body')});\n`;
  wire+=`let (client, server) = typed_support::server(vec![(${Number(o.responses.find(r=>/^2/.test(r.status)).status)}, ${o.success?'fixture["response"].clone()':'Value::Null'})], Some(auth.clone()));\n${o.success||o.unknownSuccess?'let result = ':''}client.${o.method}(${[o.params?'&parameters':null,o.bodyName?(o.bodyRequired?'&body':'Some(&body)'):null].filter(Boolean).join(', ')}).await.expect(${rust(o.id+' response')});\n`;
  if(o.success) wire+=`kalshi_client::typed::validate(${rust(o.success)}, &serde_json::to_value(result).unwrap()).expect(${rust(o.id+' roundtrip')});\n`;
  else if(o.unknownSuccess) wire+='assert!(result.body.is_null());\n';
  wire+=`let requests = server.join().unwrap();\ntyped_support::assert_request(${rust(o.id)}, &requests[0], ${o.params?'Some(&serde_json::to_value(&parameters).unwrap())':'None'}, ${o.bodyName?'Some(&serde_json::to_value(&body).unwrap())':'None'}, ${o.authenticated}, &auth);\n}\n`;
  errorWire+=`{ let fixture = &fixtures[${rust(o.id)}];\n`;
  if(o.params) errorWire+=`let parameters: ${o.params} = serde_json::from_value(fixture["parameters"].clone()).unwrap();\n`;
  if(o.bodyName) errorWire+=`let body: ${o.bodyName} = serde_json::from_value(fixture["body"].clone()).unwrap();\n`;
  for(const r of o.responses.filter(r=>!/^2/.test(r.status))){
    errorWire+=`{ let (client, server) = typed_support::server(vec![(${r.status},fixture["errors"][${rust(r.status)}].clone())],Some(auth.clone()));\nlet error = client.${o.method}(${[o.params?'&parameters':null,o.bodyName?(o.bodyRequired?'&body':'Some(&body)'):null].filter(Boolean).join(', ')}).await.expect_err(${rust(o.id+' '+r.status)});\nassert!(error.payload.is_some(), ${rust(o.id+' '+r.status+' missing typed error')});\nif let kalshi_client::KalshiError::Api(api) = error.source { assert_eq!(api.status.as_u16(),${r.status}); assert_eq!(api.headers["x-request-id"],"fake-probe"); assert_eq!(api.body,fixture["errors"][${rust(r.status)}]); } else { panic!("lost structured ApiError"); }\nserver.join().unwrap(); }\n`;
  }
  errorWire+='}\n';
}
await writeFile('src/generated/typed.rs',code);
await writeFile('tests/typed_signatures.rs',probes);
await writeFile('tests/typed_wire.rs',wire+'}\n');
await writeFile('tests/typed_errors.rs',errorWire+'}\n');
let pagination='// Generated cursor wire probes cover every paginated operation.\nmod typed_support;\nuse kalshi_client::models::*;\nuse serde_json::{Value,json};\n#[tokio::test]\nasync fn all_28_paginators_advance_keep_filters_and_terminate() {\nlet fixtures: Value=serde_json::from_str(include_str!("typed-fixtures.json")).unwrap(); let auth=typed_support::auth();\n';
for(const o of ops.filter(o=>o.paginated)) {
  pagination+=`{ let fixture=&fixtures[${rust(o.id)}]; let parameters: ${o.params}=serde_json::from_value(fixture["parameters"].clone()).unwrap(); let mut expected=serde_json::to_value(&parameters).unwrap(); let mut first=fixture["response"].clone(); first[${rust(o.cursorField)}]=json!("next cursor"); let mut last=fixture["response"].clone(); ${o.cursorRequired?`last[${rust(o.cursorField)}]=json!("");`:`last.as_object_mut().unwrap().remove(${rust(o.cursorField)});`}\nlet (client,server)=typed_support::server(vec![(200,first),(200,last)],Some(auth.clone()));\nlet mut pager=client.${o.method}_pages(parameters); assert!(pager.next_page().await.expect(${rust(o.id+' first page')}).is_some()); assert!(pager.next_page().await.expect(${rust(o.id+' last page')}).is_some()); assert!(pager.next_page().await.unwrap().is_none());\nlet requests=server.join().unwrap(); typed_support::assert_request(${rust(o.id)}, &requests[0],Some(&expected),None,${o.authenticated},&auth); expected["cursor"]=json!("next cursor"); typed_support::assert_request(${rust(o.id)}, &requests[1],Some(&expected),None,${o.authenticated},&auth); }\n`;
}
await writeFile('tests/typed_pagination.rs',pagination+'}\n');
let modelProbes='// Generated model probes cover every named schema definition and recursive inline model.\nmod model_support;\nuse kalshi_client::models::*;\nuse serde_json::Value;\n#[test]\nfn all_schema_definitions_decode_and_roundtrip() { let fixtures: Value = serde_json::from_str(include_str!("typed-model-fixtures.json")).unwrap(); let full: Value = serde_json::from_str(include_str!("typed-model-full-fixtures.json")).unwrap();\n';
for(const name of Object.keys(definitions)) modelProbes+=`for value in [&fixtures[${rust(name)}],&full[${rust(name)}]] { kalshi_client::typed::validate(${rust(name)},value).expect(${rust(name+' fixture')}); let model: ${pascal(name)} = serde_json::from_value(value.clone()).expect(${rust(name+' decode')}); let output=serde_json::to_value(model).unwrap(); kalshi_client::typed::validate(${rust(name)},&output).expect(${rust(name+' roundtrip')}); model_support::assert_presence(value,&output,${rust(name)}); }\n`;
modelProbes+='}\n#[test]\nfn recursive_null_mutations_obey_the_source_schema() { let full:Value=serde_json::from_str(include_str!("typed-model-full-fixtures.json")).unwrap(); let mut mutations=0;\n';
for(const name of Object.keys(definitions)) modelProbes+=`mutations+=model_support::assert_nulls::<${pascal(name)}>(${rust(name)},&full[${rust(name)}]);\n`;
await writeFile('tests/typed_models.rs',modelProbes+'println!("Recursive null mutations verified: {mutations}"); }\n');
let facades='// Generated domain facades delegate to the canonical typed methods.\nuse crate::client::KalshiClient;\nuse crate::generated::{models::*, typed::*};\nuse crate::typed::{TypedError, Pager, UnspecifiedResponse};\n';
const domains=new Map();
for(const operation of ops) { const domain=operation.path.split('/')[1]; if(!domains.has(domain)) domains.set(domain,[]); domains.get(domain).push(operation); }
for(const [domain, operations] of domains) {
  const name=pascal(domain)+'Api';
  facades+=`pub struct ${name}<'a> { client: &'a KalshiClient }\nimpl KalshiClient { pub fn ${domain}(&self) -> ${name}<'_> { ${name} { client: self } } }\nimpl<'a> ${name}<'a> {\n`;
  for(const o of operations){
    const args=[o.params?`parameters: &${o.params}`:null,o.bodyName?`body: ${o.bodyRequired?'&':'Option<&'}${o.bodyName}${o.bodyRequired?'':'>'}`:null].filter(Boolean).join(', ');
    facades+=`pub async fn ${o.method}(&self${args?', '+args:''}) -> Result<${o.success??(o.unknownSuccess?'UnspecifiedResponse':'()')},TypedError<${o.type}Error>> { self.client.${o.method}(${[o.params?'parameters':null,o.bodyName?'body':null].filter(Boolean).join(', ')}).await }\n`;
    if(o.paginated) facades+=`pub fn ${o.method}_pages(&self, parameters: ${o.params}) -> Pager<'a,${o.params},${o.success},${o.type}Error> { self.client.${o.method}_pages(parameters) }\n`;
  }
  facades+='}\n';
}
await writeFile('src/generated/domains.rs',facades);
await writeFile('tools/typed-coverage.json',JSON.stringify({sourceSha256:bundle.sources.openapi.sha256,sourceDocumentSha256:createHash('sha256').update(JSON.stringify(source)).digest('hex'),operationCount:ops.length,paginationCount:ops.filter(o=>o.paginated).length,componentSchemaCount:Object.keys(source.components.schemas).length,definitionCount:Object.keys(definitions).length,generatorCompatibility:["Optional/default fields preserve presence instead of injecting source defaults", "Required fields never deserialize through defaults", "Optional nullable fields preserve missing/null/value", "Required nullable fields require presence", "Every model Debug is redacted; unused default helpers removed", "Invalid enum defaults are omitted only from generator input, never schema constraints", "Undeclared response bodies are unknown; only 204 is guaranteed empty"],compatibility:compat,operations:ops.map(({id,method,source,params,bodyName,success,successStatus,unknownSuccess,responses,parameters,paginated})=>({id,method,source,parameters:params,parameterBindings:parameters.map(p=>({name:p.name,in:p.in,style:p.style??'form',explode:p.explode??true,array:resolve(p.schema).type==='array'})),body:bodyName,success,successStatus,unknownSuccess,errors:responses.filter(r=>!/^2/.test(r.status)).map(({status,name})=>({status,schema:name})),paginated}))},null,2)+'\n');
console.log(`Generated ${ops.length} named async methods, ${ops.filter(o=>o.paginated).length} paginators, ${Object.keys(definitions).length} schema definitions`);
// Prettyplease block documentation requires a second rustfmt pass to stabilize.
const generatedRust = ['src/generated/models.rs','src/generated/typed.rs','src/generated/domains.rs','tests/typed_signatures.rs','tests/typed_wire.rs','tests/typed_errors.rs','tests/typed_pagination.rs','tests/typed_models.rs'];
for(let pass=0;pass<2;pass++) { const format=spawnSync('rustfmt',['--edition','2024','--config','skip_children=true',...generatedRust],{stdio:'inherit'}); if(format.status!==0) process.exit(format.status??1); }
