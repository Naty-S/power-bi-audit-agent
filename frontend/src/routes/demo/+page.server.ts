import { env } from "$env/dynamic/private";
import type { PageServerLoad } from './$types';


export const load = (async () => {

  return { api_url: env.VITE_API_URL }

}) satisfies PageServerLoad;
