import React from 'react';
import { cn } from '@/shared/lib/cn';

interface DropZoneProps {
  onFiles: (files: File[]) => void;
}

export const DropZone: React.FC<DropZoneProps> = ({ onFiles }) => {
  const inputRef = React.useRef<HTMLInputElement | null>(null);

  const handleDrop = (ev: React.DragEvent<HTMLDivElement>) => {
    ev.preventDefault();
    const list = ev.dataTransfer.files;
    onFiles(Array.from(list));
  };

  const handleSelect = (ev: React.ChangeEvent<HTMLInputElement>) => {
    if (!ev.target.files) return;
    onFiles(Array.from(ev.target.files));
  };

  return (
    <React.Fragment>
      <div
        onDragOver={(e) => e.preventDefault()}
        onDrop={handleDrop}
        className={cn(
          'border-2 border-dashed border-slate-600 rounded-lg p-6',
          'bg-slate-900/60 hover:border-blue-500 transition-colors cursor-pointer'
        )}
        onClick={() => inputRef.current?.click()}
      >
        <p className="text-center text-slate-200">드래그 앤 드랍 또는 클릭해 파일 선택</p>
      </div>
      <input
        ref={inputRef}
        type="file"
        multiple
        accept="image/png,image/jpeg,image/webp"
        className="hidden"
        onChange={handleSelect}
      />
    </React.Fragment>
  );
};
