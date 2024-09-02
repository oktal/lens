import type { StreamId } from "$lib/lens/types";
import { queriesStore } from "$lib/stores/queries.svelte";
import type { QueryStream } from "$lib/stores/QueryStream.svelte";

export type SplitDirection = 'vertical' | 'horizontal';

type PaneInfo = {
  query?: string,
  stream?: QueryStream,
} | undefined;

export class QueryPaneGroup {
  direction = $state<SplitDirection | undefined>(undefined);
  panes = $state<PaneInfo[]>([undefined]);
  overlayVisible = $state(false);

  constructor() {
  }

  split(direction: SplitDirection) {
    this.direction = direction;

    if (this.panes.length == 1) {
      this.panes.push(undefined);
    }
  }

  close(paneId: number) {
    if (paneId >= this.panes.length)
      throw new Error(`invalid pane ${paneId}`);

    this.panes = this.panes.splice(paneId, 1);
  }

  renew(paneId: number, streamId: StreamId) {
    if (paneId >= this.panes.length)
      throw new Error(`invalid pane ${paneId}`);

    const stream = queriesStore.get(streamId);
    if (!stream)
      throw new Error(`Could not find stream for id ${streamId}`);

    if (stream.kind === 'partial') {
      this.panes[paneId] = {
        query: stream.query,
        stream: undefined
      }
    } else if (stream.kind === 'full') {
      this.panes[paneId] = {
        query: stream.stream.query,
        stream: stream.stream
      }
    }
  }

  save(paneId: number) {
    if (paneId >= this.panes.length)
      throw new Error(`invalid pane ${paneId}`);

    const stream = this.panes[paneId]?.stream;
    if (stream)
      queriesStore.save(stream);
  }

  clear(paneId: number) {
    if (paneId >= this.panes.length)
      throw new Error(`invalid pane ${paneId}`);

    this.panes[paneId] = undefined;

  }

  async run(paneId: number, query: string): Promise<QueryStream> {
    if (paneId >= this.panes.length)
      throw new Error(`invalid pane ${paneId}`);

    const stream = await queriesStore.run(query);
    this.panes[paneId] = {
      query,
      stream
    }
    return stream;
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
