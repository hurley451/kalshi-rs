import { createHash } from 'node:crypto';

const sourceSha256 = '091f171718a0376b0c1a2e9348b02b1ce003d0786dd04d772dd11005ca414bf4';
const facts = [
  { id: 'ioc-excludes-expiry', property: 'expiration_time', descriptionSha256: 'ca248a7305b84d6dfb5669451a0038b88606a3a4b75df0a544ff12ea4d7dca1e' },
  { id: 'reduce-only-requires-ioc', property: 'reduce_only', descriptionSha256: '0e369a2964e6e80ef7b6f5d70e9dff702ce7c3da0ab097a4264fdd13743927cf' },
];
const schemaPrerequisites = {
  CreateOrderV2Request: '42525b65074f223f677d484d70af0ab48b34e4700ead1938c23c4ad0e7a8a6da',
  BatchCreateOrdersV2Request: '8ed315f8761b4cf9a8205f1931cc8da41759cda093ba103171e5e710f10f4c0c',
};

export function qualifyPredictionsOrderRules(source, hash) {
  if (hash !== sourceSha256) throw Error('V2 order rules require the reviewed source hash');
  const model = source.components?.schemas?.CreateOrderV2Request;
  for (const fact of facts) {
    const description = model?.properties?.[fact.property]?.description;
    if (typeof description !== 'string' || createHash('sha256').update(description).digest('hex') !== fact.descriptionSha256) {
      throw Error(`V2 order rule source changed: ${fact.id}`);
    }
  }
  for (const [name, expected] of Object.entries(schemaPrerequisites)) {
    const actual = createHash('sha256').update(JSON.stringify(source.components.schemas[name])).digest('hex');
    if (actual !== expected) throw Error(`V2 order model prerequisites changed: ${name}`);
  }
  if (model.properties.expiration_time.type !== 'integer' || model.properties.expiration_time.format !== 'int64'
      || model.properties.expiration_time.nullable === true
      || model.properties.reduce_only.type !== 'boolean'
      || model.properties.time_in_force.type !== 'string'
      || JSON.stringify(model.properties.time_in_force.enum) !== JSON.stringify(['fill_or_kill', 'good_till_canceled', 'immediate_or_cancel'])) {
    throw Error('V2 order rule field shapes changed');
  }
  const bindings = [
    { operationId: 'CreateOrderV2', path: '/portfolio/events/orders', model: 'CreateOrderV2Request', memberPointer: '' },
    { operationId: 'BatchCreateOrdersV2', path: '/portfolio/events/orders/batched', model: 'BatchCreateOrdersV2Request', memberPointer: '/orders/*' },
  ];
  for (const binding of bindings) {
    const operation = source.paths?.[binding.path]?.post;
    if (operation?.operationId !== binding.operationId || operation.requestBody?.required !== true || operation.requestBody?.content?.['application/json']?.schema?.$ref !== `#/components/schemas/${binding.model}`) {
      throw Error(`V2 order rule binding changed: ${binding.operationId}`);
    }
  }
  const orders = source.components.schemas.BatchCreateOrdersV2Request?.properties?.orders;
  if (orders?.type !== 'array' || orders.items?.$ref !== '#/components/schemas/CreateOrderV2Request') {
    throw Error('V2 batch member binding changed');
  }
  return {
    sourceId: 'kalshi-openapi-rest', sourceSha256, schemaPrerequisites,
    rules: facts.map(({ property, ...fact }) => ({ ...fact, pointer: `#/components/schemas/CreateOrderV2Request/properties/${property}/description` })),
    bindings,
  };
}
