import { atom } from 'jotai';
import { defaultOptions, type Options } from '@/entities/options';

export const optionsAtom = atom<Options>(defaultOptions);
