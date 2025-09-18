type RemoveText = (input: Uint8Array) => Promise<Uint8Array>;
type FilterOperations = (
  input: Uint8Array,
  operationsToRemove: string[]
) => Promise<Uint8Array>;

interface Cleaner {
  removeText: RemoveText;
  filterOperations: FilterOperations;
}

export function cleaner(): Promise<Cleaner>;
