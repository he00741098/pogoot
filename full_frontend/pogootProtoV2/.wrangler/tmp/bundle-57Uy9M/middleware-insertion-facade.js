				import worker, * as OTHER_EXPORTS from "/home/he00741098/Documents/GitHub/pogoot/full_frontend/pogootProtoV2/node_modules/.pnpm/wrangler@3.50.0_@cloudflare+workers-types@4.20240405.0/node_modules/wrangler/templates/pages-shim.ts";
				import * as __MIDDLEWARE_0__ from "/home/he00741098/Documents/GitHub/pogoot/full_frontend/pogootProtoV2/node_modules/.pnpm/wrangler@3.50.0_@cloudflare+workers-types@4.20240405.0/node_modules/wrangler/templates/middleware/middleware-ensure-req-body-drained.ts";
import * as __MIDDLEWARE_1__ from "/home/he00741098/Documents/GitHub/pogoot/full_frontend/pogootProtoV2/node_modules/.pnpm/wrangler@3.50.0_@cloudflare+workers-types@4.20240405.0/node_modules/wrangler/templates/middleware/middleware-miniflare3-json-error.ts";
				
				worker.middleware = [
					__MIDDLEWARE_0__.default,__MIDDLEWARE_1__.default,
					...(worker.middleware ?? []),
				].filter(Boolean);
				
				export * from "/home/he00741098/Documents/GitHub/pogoot/full_frontend/pogootProtoV2/node_modules/.pnpm/wrangler@3.50.0_@cloudflare+workers-types@4.20240405.0/node_modules/wrangler/templates/pages-shim.ts";
				export default worker;