import * as backend from '$lib/server/backend';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
  const datasets: backend.Dataset[] = await backend.list_datasets()
  return {
    datasets
  };
};
