import type { StreamId } from "$lib/lens/types";
import { useQueryStream, type QueryStream } from "./QueryStream.svelte";

type StreamInfo = {
  id: StreamId;
  query: string;
}

type Stream = {
  kind: 'partial',
  id: StreamId;
  query: string;
} |
{ kind: 'full', stream: QueryStream };


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
    return stream;
  }

  save(stream: QueryStream): boolean {
    let index = this.streams.findIndex(s => stream.streamId === this.getStreamId(s));
    if (index === undefined)
      return false;

    this.streams[index] = {
      kind: 'full',
      stream
    };

    return true;
  }

  delete(id: StreamId): boolean {
    const oldLen = this.streams.length;
    this.streams = this.streams.filter(s => id !== this.getStreamId(s));
    return oldLen > this.streams.length;
  }

  get(id: StreamId): Stream | undefined {
    return this.streams.find(s => id === this.getStreamId(s));
  }

  getStreamInfo(stream: Stream): StreamInfo {
    if (stream.kind === 'partial') {
      const { id, query } = stream;
      return { id, query };
    }
    else if (stream.kind === 'full') {
      const { streamId: id, query } = stream.stream;
      return { id, query };

    }

    throw new Error("Can't determine information for stream");
  }

  getStreamId(stream: Stream): StreamId {
    return this.getStreamInfo(stream).id;
  }
}

const queriesStore = new QueriesStore();
export { queriesStore }
