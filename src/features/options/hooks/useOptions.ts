import { useAtom } from 'jotai';
import { optionsAtom } from '@/features/options/state/options.atom';

export const useOptions = () => {
  const [options, setOptions] = useAtom(optionsAtom);

  const updateOption = <K extends keyof typeof options>(key: K, value: (typeof options)[K]) => {
    setOptions((prev) => ({ ...prev, [key]: value }));
  };

  return { options, updateOption };
};
