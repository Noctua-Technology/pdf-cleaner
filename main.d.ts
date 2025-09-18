type RemoveText = (input: Uint8Array) => Promise<Uint8Array>;
type FilterOperations = (
  input: Uint8Array,
  operationsToRemove: string[]
) => Promise<Uint8Array>;

export function cleaner(): Promise<{
  removeText: RemoveText;
  filterOperations: FilterOperations;
}>;
