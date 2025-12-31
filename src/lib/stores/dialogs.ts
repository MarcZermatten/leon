import { writable } from 'svelte/store';

interface ConfirmDialogState {
	show: boolean;
	title: string;
	message: string;
	confirmText: string;
	cancelText: string;
	variant: 'danger' | 'warning' | 'info';
	resolve: ((value: boolean) => void) | null;
}

interface InputDialogState {
	show: boolean;
	title: string;
	message: string;
	placeholder: string;
	defaultValue: string;
	confirmText: string;
	cancelText: string;
	resolve: ((value: string | null) => void) | null;
}

interface AlertDialogState {
	show: boolean;
	title: string;
	message: string;
	variant: 'success' | 'error' | 'warning' | 'info';
	resolve: (() => void) | null;
}

export const confirmDialog = writable<ConfirmDialogState>({
	show: false,
	title: '',
	message: '',
	confirmText: 'Confirmer',
	cancelText: 'Annuler',
	variant: 'info',
	resolve: null
});

export const inputDialog = writable<InputDialogState>({
	show: false,
	title: '',
	message: '',
	placeholder: '',
	defaultValue: '',
	confirmText: 'OK',
	cancelText: 'Annuler',
	resolve: null
});

export const alertDialog = writable<AlertDialogState>({
	show: false,
	title: '',
	message: '',
	variant: 'info',
	resolve: null
});

// Helper functions
export function showConfirm(options: {
	title?: string;
	message: string;
	confirmText?: string;
	cancelText?: string;
	variant?: 'danger' | 'warning' | 'info';
}): Promise<boolean> {
	return new Promise((resolve) => {
		confirmDialog.set({
			show: true,
			title: options.title || 'Confirmation',
			message: options.message,
			confirmText: options.confirmText || 'Confirmer',
			cancelText: options.cancelText || 'Annuler',
			variant: options.variant || 'info',
			resolve
		});
	});
}

export function showInput(options: {
	title?: string;
	message: string;
	placeholder?: string;
	defaultValue?: string;
	confirmText?: string;
	cancelText?: string;
}): Promise<string | null> {
	return new Promise((resolve) => {
		inputDialog.set({
			show: true,
			title: options.title || 'Saisie',
			message: options.message,
			placeholder: options.placeholder || '',
			defaultValue: options.defaultValue || '',
			confirmText: options.confirmText || 'OK',
			cancelText: options.cancelText || 'Annuler',
			resolve
		});
	});
}

export function showAlert(options: {
	title?: string;
	message: string;
	variant?: 'success' | 'error' | 'warning' | 'info';
}): Promise<void> {
	return new Promise((resolve) => {
		alertDialog.set({
			show: true,
			title: options.title || 'Information',
			message: options.message,
			variant: options.variant || 'info',
			resolve
		});
	});
}
