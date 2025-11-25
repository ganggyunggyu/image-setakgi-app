import { invoke } from '@tauri-apps/api/tauri';
import { Options } from '@/entities/options';

export interface PreviewRequest {
  options: Options;
  imageBytes: number[];
  saturation?: number;
}

export interface PreviewResponse {
  bytes: number[];
}

export interface FilePayload {
  name: string;
  bytes: number[];
}

export interface ConvertResult {
  outputDir: string;
  succeeded: number;
  failed: number;
}

export const fetchPreview = async (payload: PreviewRequest) => {
  return invoke<PreviewResponse>('generate_preview', { payload });
};

export const convertAll = async (
  options: Options,
  files: FilePayload[],
  outputRoot: string,
  saturation?: number
) => {
  return invoke<ConvertResult>('convert_all', { options, files, outputRoot, saturation });
};

export const savePreset = async (name: string, options: Options) => {
  return invoke<void>('save_preset', { name, options });
};

export const loadPreset = async (name: string) => {
  return invoke<Options>('load_preset', { name });
};
