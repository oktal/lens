import type { StreamId } from "$lib/lens/types";
import { queriesStore, type QueryStoreEntry } from "$lib/stores/queries.svelte";
import type { QueryStream } from "$lib/stores/QueryStream.svelte";

export type SplitDirection = 'vertical' | 'horizontal';
export const DEFAULT_QUERY_TITLE: string = 'Unnamed';

import { Stopwatch } from '$lib/Stopwatch.svelte'

class QueryPane {
  query = $state('');
  title = $state('');
  stream = $state<QueryStream | undefined>(undefined);
  stopWatch = new Stopwatch();

  streamId?: StreamId;

  constructor(queryString?: string, title: string = DEFAULT_QUERY_TITLE) {
    this.query = queryString ?? '';
    this.title = title;
  }

  clear() {
    this.query = '';
    this.title = DEFAULT_QUERY_TITLE;

    this.stopWatch.reset();
    this.streamId = undefined;
    this.stream = undefined;
  }

  close() {
    this.stopWatch.stop();
  }

  renew(entry: QueryStoreEntry) {
    const { title, stream } = entry;

    this.title = title;

    if (stream.kind === 'partial') {
      this.query = stream.query;
      this.streamId = stream.id;
      this.stream = undefined;
    } else if (stream.kind === 'full') {
      const { query, streamId } = stream.stream;

      this.query = query;
      this.streamId = streamId;
      this.stream = stream.stream;
    }
  }
}

export class QueryPaneGroup {
  direction = $state<SplitDirection | undefined>(undefined);
  panes = $state<QueryPane[]>([new QueryPane('')]);
  overlayVisible = $state(false);

  constructor() {
  }

  split(direction: SplitDirection) {
    this.direction = direction;

    if (this.panes.length == 1) {
      this.panes.push(new QueryPane(''))
    }
  }

  close(paneId: number) {
    if (paneId >= this.panes.length)
      throw new Error(`invalid pane ${paneId}`);

    this.panes[paneId].close();
    this.panes.splice(paneId, 1);
  }

  renew(paneId: number, streamId: StreamId) {
    if (paneId >= this.panes.length)
      throw new Error(`invalid pane ${paneId}`);

    const entry = queriesStore.get(streamId);
    if (!entry)
      throw new Error(`Could not find query for stream ${streamId}`);

    this.panes[paneId].renew(entry);
  }

  save(paneId: number) {
    if (paneId >= this.panes.length)
      throw new Error(`invalid pane ${paneId}`);

    const stream = this.panes[paneId]?.stream;
    if (stream)
      queriesStore.save(stream);
  }

  setTitle(paneId: number) {
    if (paneId >= this.panes.length)
      throw new Error(`invalid pane ${paneId}`);

    const { streamId, title } = this.panes[paneId];

    if (streamId)
      queriesStore.setTitle(streamId, title);
  }

  clear(paneId: number) {
    if (paneId >= this.panes.length)
      throw new Error(`invalid pane ${paneId}`);

    this.panes[paneId].clear();
  }

  async run(paneId: number): Promise<QueryStream> {
    if (paneId >= this.panes.length)
      throw new Error(`invalid pane ${paneId}`);

    const { query, title } = this.panes[paneId];
    const stream = await queriesStore.run(query, title);

    this.panes[paneId].stopWatch.restart();
    this.panes[paneId].streamId = stream.streamId;
    this.panes[paneId].stream = stream;

    return stream;
  }

  pause(paneId: number) {
    if (paneId >= this.panes.length)
      throw new Error(`invalid pane ${paneId}`);

    const pane = this.panes[paneId];
    pane.stopWatch.pause();
    if (pane.stream)
      pane.stream.pause();
  }

  stop(paneId: number) {
    if (paneId >= this.panes.length)
      throw new Error(`invalid pane ${paneId}`);

    const pane = this.panes[paneId];
    pane.stopWatch.stop();
    if (pane.stream)
      pane.stream.stop();
  }

  showOverlay() {
    this.overlayVisible = true;
  }

  hideOverlay() {
    this.overlayVisible = false;
  }

  toggleOverlay(visible: boolean) {
    if (visible)
      this.showOverlay();
    else
      this.hideOverlay();
  }
}

const queryPaneGroup = new QueryPaneGroup();
export { queryPaneGroup }
