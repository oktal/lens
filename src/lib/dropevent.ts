import { listen } from '@tauri-apps/api/event'

class DropEvent extends DragEvent {
  offsetX: number
  offsetY: number

  constructor(path: string, offsetX: number, offsetY: number) {
    super('drop', {
      dataTransfer: new DataTransfer(),
    })

    this.offsetX = offsetX
    this.offsetY = offsetY
    this.dataTransfer?.setData('text/plain', path)
  }
}

const startListeningForDragAndDropEvents = () => {
  const appContainer = document.body;

  let draggedData: string | null = null
  let dropTarget: HTMLElement | null = null
  let offsetX = 0
  let offsetY = 0

  appContainer?.addEventListener('dragstart', e => {
    draggedData = e.dataTransfer?.getData('text/plain') || null
  })

  appContainer?.addEventListener('dragover', e => {
    offsetX = e.offsetX
    offsetY = e.offsetY
  })

  appContainer?.addEventListener('dragenter', e => {
    dropTarget = e.target as HTMLElement
  })

  appContainer?.addEventListener('dragend', () => {
    dropTarget = null
    draggedData = null
  })

  listen('tauri://file-drop', e => {
    let filePath = draggedData || ''

    if (Array.isArray(e.payload) && typeof e.payload[0] === 'string') {
      filePath = e.payload[0]
    }

    const dropEvent = new DropEvent(filePath, offsetX, offsetY)
    dropEvent.dataTransfer?.setData('text/plain', filePath)
    dropTarget?.dispatchEvent(dropEvent)

    dropTarget = null
    draggedData = null
  })
}

export { DropEvent, startListeningForDragAndDropEvents }
