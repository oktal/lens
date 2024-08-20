import { client } from '$lib/lens/api'
import type { StreamId } from '$lib/lens/types';

type State = 'running' | 'paused' | 'stopped' | 'finished' | undefined;

export interface QueryStream {
  query: string;
  streamId: StreamId;
  columns: string[];

  get rows(): string[][];
  get state(): State; 

  hasNext: boolean;
  fetchNext(): Promise<void>;

  pause(): void;
  resume(): void;
  stop(): void;
}

export async function useQueryStream(query: string): Promise<QueryStream> {
  const streamId = await client.sql.stream(query);
  const firstBatch = await client.sql.next(streamId);

  if (firstBatch?.length > 0) {
    const columns = firstBatch[0].columns;
    let rows = $state(firstBatch.map(r => r.values));
    let state = $state<State>('running');

    let hasNext = true;

    const fetchNext = async () => {
      if (state === 'stopped')
        return;

      state = 'running';
      const nextBatch = await client.sql.next(streamId);

      if (nextBatch?.length > 0) {
        rows.push(...nextBatch.map(r => r.values));
      }
      else {
        hasNext = false;
        state = 'finished';
      }
    };

    const pause = () => {
      if (state === 'running') {
        state = 'paused'; };
    };
    const resume = () => {
      if (state === 'paused')
        state = 'running';
    };

    const stop = () => {
      if (state === 'running' || state === 'paused') {
        state = 'stopped';
        hasNext = false;
      }
    };

    return {
      query,
      streamId,
      columns,
      get rows() { return rows },
      get state() { return state },
      hasNext,
      fetchNext,
      pause,
      resume,
      stop,
    };
  } else {
    return {
      query,
      streamId,
      columns: [],
      get rows() { return [] },
      get state() { return undefined },
      hasNext: false,
      fetchNext: (): Promise<void> => Promise.resolve(),
      pause: () => {},
      resume: () => {},
      stop: () => {},
    }
  }
}
