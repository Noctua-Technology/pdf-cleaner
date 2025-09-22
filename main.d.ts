type RemoveText = (input: Uint8Array) => Promise<Uint8Array>;

type LeaveOnlyText = (input: Uint8Array) => Promise<Uint8Array>;

type FilterOperations = (
  input: Uint8Array,
  operationsToRemove: string[]
) => Promise<Uint8Array>;

interface Cleaner {
  filterOperations: FilterOperations;
  leaveOnlyText: LeaveOnlyText;
  removeText: RemoveText;
}

export function cleaner(): Promise<Cleaner>;
