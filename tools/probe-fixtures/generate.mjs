import jsf from 'json-schema-faker';
import RefParser from 'json-schema-ref-parser';
import { readFile, writeFile } from 'node:fs/promises';
const schema=JSON.parse(await readFile('src/generated/typed-schema.json','utf8'));
const dereferenced=await RefParser.dereference(structuredClone(schema));
const coverage=JSON.parse(await readFile('tools/typed-coverage.json','utf8'));
jsf.option({alwaysFakeOptionals:false,optionalsProbability:0,requiredOnly:true,useDefaultValue:false,useExamplesValue:true,random:()=>0.5});
const sample=name=>jsf.generate(dereferenced.definitions[name]);
const fixtures={};
for(const op of coverage.operations){
  const fixture={};
  for(const [key,name] of Object.entries({parameters:op.parameters,body:op.body,response:op.success})) {
    if(name) fixture[key]=sample(name);
  }
  fixture.errors=Object.fromEntries(op.errors.map(({status,schema:name})=>[status,name?sample(name):null]));
  fixtures[op.id]=fixture;
}
await writeFile('tests/typed-fixtures.json',JSON.stringify(fixtures,null,2)+'\n');
console.log(`Generated schema-based request/response fixtures for ${Object.keys(fixtures).length} operations`);
const models={};
for(const name of Object.keys(schema.definitions)) models[name]=sample(name);
await writeFile('tests/typed-model-fixtures.json',JSON.stringify(models,null,2)+'\n');
console.log(`Generated fixtures for ${Object.keys(models).length} schema definitions`);
let seed=42;
jsf.option({requiredOnly:false,alwaysFakeOptionals:true,optionalsProbability:1,fillProperties:true,maxItems:2,random:()=> { seed=(Math.imul(seed,1664525)+1013904223)>>>0; return seed/4294967296; }});
const full={};
for(const name of Object.keys(schema.definitions)) full[name]=sample(name);
await writeFile('tests/typed-model-full-fixtures.json',JSON.stringify(full,null,2)+'\n');
console.log(`Generated fixtures including optional fields for ${Object.keys(full).length} definitions`);
