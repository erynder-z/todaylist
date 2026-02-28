export type FolderValidation = {
	is_valid: boolean;
	is_writable: boolean;
	exists: boolean;
	note_count: number;
	error: string | null;
};
