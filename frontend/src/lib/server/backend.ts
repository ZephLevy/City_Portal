type GraphQLResponse<T> = {
  data?: T;
  errors?: Array<{
    message: string;
    locations?: { line: number; column: number }[];
    path?: string[];
  }>;
};

export type Dataset = {
  id: number;
  name: string;
  description: string;
};

export type Location = {
  id: number,
  name: string,
  lat: number,
  lon: number;
};

export type Record = {
  id: number,
  dataset_id: number,
  location_id: number | undefined,
  timestamp: string,
  data: string;
};

async function query_graphql<T>(query: string): Promise<T> {
  const response = await fetch('http://host.containers.internal:3000/graphql', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ query }),
  });

  if (!response.ok) {
    throw new Error(`Error - status ${response.status}`);
  }

  const result = await response.json() as GraphQLResponse<T>;

  if (result.errors) {
    throw new Error(result.errors.map(e => e.message).join(', '));
  }

  return result.data!;
} 

export async function list_datasets(): Promise<Dataset[]> {
  const query = `
    query {
      datasets {
        id
        name
        description
      }
    }
  `;
  return (await query_graphql<{ datasets: Dataset[] }>(query)).datasets;
}

export async function query_dataset(id: string): Promise<Record> {
  // TODO: Maybe just interpolating the id like this isn't the best way to do it.
  const query = `
    query {
      queryDataset(id: ${id}) {
        id
        locationId
        data
      }
    }
  `;

  return (await query_graphql<{
    queryDataset: Record,
  }>(query)).queryDataset;
}
