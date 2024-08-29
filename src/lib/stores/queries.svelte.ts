import type { StreamId } from "$lib/lens/types";
import { useQueryStream, type QueryStream } from "./QueryStream.svelte";

type PartialStream = {
  kind: 'partial',
  query: string;
  id: StreamId;
}

type Stream = PartialStream | { kind: 'full', id: StreamId, query: string, stream: QueryStream };

class QueriesStore {
  private maxHistory?: number;
  streams = $state<Stream[]>([]);

  constructor(maxHistory?: number) {
    this.maxHistory = maxHistory;
  }

  async run(query: string): Promise<QueryStream> {
    const addToHistory = ({ stream }: { stream: QueryStream }) => {
      const { streamId: id, query } = stream;

      this.streams.push({
        kind: 'partial',
        query,
        id
      });

      if (this.maxHistory && this.streams.length >= this.maxHistory) {
        this.streams.splice(0, 1);
      }
    };

    const stream = await useQueryStream(query);
    addToHistory({ stream });

    console.log(this.streams);

    return stream;
  }

  save(stream: QueryStream): boolean {
    let index = this.streams.findIndex(s => s.id === stream.streamId);
    if (index === undefined)
      return false;

    const { streamId: id, query } = stream;

    this.streams[index] = {
      kind: 'full',
      id,
      query,
      stream
    };

    return true;
  }

  delete(id: StreamId): boolean {
    const oldLen = this.streams.length;
    this.streams = this.streams.filter(s => s.id !== id);
    return oldLen > this.streams.length;
  }
}

const queriesStore = new QueriesStore();
export { queriesStore }
