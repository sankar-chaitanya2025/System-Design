export interface WorkbenchHotkeyBindings {
  onTogglePlayPause: () => void;
  onStepSingleEvent: () => void;
  onDeleteSelected: () => void;
  onUndo: () => void;
  onRedo: () => void;
}

export function useWorkbenchHotkeys(_bindings: WorkbenchHotkeyBindings): void {
  throw new Error('Not implemented');
}
