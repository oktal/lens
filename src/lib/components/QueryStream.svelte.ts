import { client } from '$lib/lens/api'
import type { StreamId } from '$lib/lens/types';

export interface QueryStream {
  query: string;
  streamId: StreamId;
  columns: string[];

  get rows(): string[][];

  hasNext: boolean;
  fetchNext(): Promise<void>;
}

export async function useQueryStream(query: string): Promise<QueryStream> {
  const streamId = await client.sql.stream(query);
  const firstBatch = await client.sql.next(streamId);

  if (firstBatch?.length > 0) {
    const columns = firstBatch[0].columns;
    let rows = $state(firstBatch.map(r => r.values));
    let hasNext = true;

    const fetchNext = async () => {
      const nextBatch = await client.sql.next(streamId);

      if (nextBatch?.length > 0) {
        rows.push(...nextBatch.map(r => r.values));
      }
      else {
        hasNext = false;
      }
    };

    return {
      query,
      streamId,
      columns,
      get rows() { return rows },
      hasNext,
      fetchNext,
    };
  } else {
    return {
      query,
      streamId,
      columns: [],
      get rows() { return [] },
      hasNext: false,
      fetchNext: (): Promise<void> => Promise.resolve()
    }
  }

}
