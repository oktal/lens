import type { StreamId } from "$lib/lens/types";
import { useQueryStream, type QueryStream } from "./QueryStream.svelte";

type StreamInfo = {
  id: StreamId;
  query: string;
}

export type QueryStoreStream = {
  kind: 'partial',
  id: StreamId;
  query: string;
} |
{ kind: 'full', stream: QueryStream };

export class QueryStoreEntry {
  title = $state('');
  query: string;
  stream: QueryStoreStream;

  constructor(stream: QueryStream, title: string) {
    const { streamId: id, query } = stream;

    this.title = title;
    this.query = query;

    this.stream = {
      kind: 'partial',
      id,
      query,
    }
  }
}

class QueriesStore {
  private maxHistory?: number;
  entries = $state<QueryStoreEntry[]>([]);

  constructor(maxHistory?: number) {
    this.maxHistory = maxHistory;
  }

  async run(query: string, title: string): Promise<QueryStream> {
    const addToHistory = ({ stream }: { stream: QueryStream }) => {
      this.entries.push(new QueryStoreEntry(stream, title));

      if (this.maxHistory && this.entries.length >= this.maxHistory) {
        this.entries.splice(0, 1);
      }
    };

    const stream = await useQueryStream(query);
    addToHistory({ stream });
    return stream;
  }

  setTitle(streamId: StreamId, title: string) {
    let index = this.entries.findIndex(e => streamId === this.getStreamId(e.stream));
    if (index === undefined)
      return;

    this.entries[index].title = title;
  }

  save(stream: QueryStream): boolean {
    let index = this.entries.findIndex(e => stream.streamId === this.getStreamId(e.stream));
    if (index === undefined)
      return false;

    this.entries[index].stream = {
      kind: 'full',
      stream
    };

    return true;
  }

  delete(id: StreamId): boolean {
    const oldLen = this.entries.length;
    this.entries = this.entries.filter(e => id !== this.getStreamId(e.stream));
    return oldLen > this.entries.length;
  }

  get(id: StreamId): QueryStoreEntry | undefined {
    return this.entries.find(e => id === this.getStreamId(e.stream));
  }

  getStreamInfo(stream: QueryStoreStream): StreamInfo {
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

  getStreamId(stream: QueryStoreStream): StreamId {
    return this.getStreamInfo(stream).id;
  }
}

const queriesStore = new QueriesStore();
export { queriesStore }
