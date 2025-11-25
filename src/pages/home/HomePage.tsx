import React from 'react';
import { useMutation } from '@tanstack/react-query';
import { convertAll, fetchPreview } from '@/shared/api/tauri';
import { cn } from '@/shared/lib/cn';
import { Button } from '@/shared/ui';
import { useOptions } from '@/features/options/hooks/useOptions';
import { useUploadList } from '@/features/uploader/hooks/useUploadList';
import { DropZone } from '@/features/uploader/ui/DropZone';
import { defaultOptions } from '@/entities/options';

export const HomePage: React.FC = () => {
  const { options, updateOption } = useOptions();
  const { files, addFiles, removeFile, clearFiles } = useUploadList();
  const [previewUrl, setPreviewUrl] = React.useState<string | null>(null);
  const [outputDir, setOutputDir] = React.useState<string>('');

  const previewMutation = useMutation({
    mutationKey: ['preview'],
    mutationFn: async (file: File) => {
      const bytes = new Uint8Array(await file.arrayBuffer());
      const res = await fetchPreview({
        options,
        imageBytes: Array.from(bytes),
      });
      const blob = new Blob([new Uint8Array(res.bytes)], { type: 'image/png' });
      return URL.createObjectURL(blob);
    },
    onSuccess: (url) => setPreviewUrl(url),
  });

  const convertMutation = useMutation({
    mutationKey: ['convert'],
    mutationFn: async () => {
      const payload = await Promise.all(
        files.map(async (item) => {
          const bytes = new Uint8Array(await item.file.arrayBuffer());
          return { name: item.file.name, bytes: Array.from(bytes) };
        })
      );
      return convertAll(options, payload, outputDir || '.', 1.0);
    },
  });

  const handlePreview = (file: File) => {
    previewMutation.mutate(file);
  };

  const handleReset = () => {
    clearFiles();
    setPreviewUrl(null);
  };

  return (
    <React.Fragment>
      <main className="min-h-screen bg-slate-950 text-slate-50 p-8">
        <header className="flex items-center justify-between mb-6">
          <div>
            <h1 className="text-2xl font-bold">Image Setakgi (Rust + Tauri)</h1>
            <p className="text-slate-400 text-sm">오프라인 고성능 이미지 변환기</p>
          </div>
          <div className="flex gap-2">
            <Button variant="ghost" onClick={handleReset}>
              목록 초기화
            </Button>
            <Button onClick={() => convertMutation.mutate()} disabled={convertMutation.isPending || files.length === 0}>
              변환 실행
            </Button>
          </div>
        </header>
        <section className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div className="col-span-2 space-y-4">
            <DropZone onFiles={addFiles} />
            <div className="bg-slate-900 rounded-lg p-4">
              <div className="flex items-center justify-between mb-3">
                <h2 className="font-semibold">이미지 리스트</h2>
                <span className="text-sm text-slate-400">{files.length} files</span>
              </div>
              <div className="space-y-2 max-h-80 overflow-auto">
                {files.map((item) => (
                  <div
                    key={item.id}
                    className={cn(
                      'flex items-center justify-between rounded-md px-3 py-2',
                      'bg-slate-800 hover:bg-slate-700 transition-colors'
                    )}
                  >
                    <div className="flex flex-col">
                      <span className="font-medium">{item.file.name}</span>
                      <span className="text-xs text-slate-400">
                        {(item.file.size / 1024 / 1024).toFixed(2)} MB
                      </span>
                    </div>
                    <div className="flex gap-2">
                      <Button variant="ghost" onClick={() => handlePreview(item.file)}>
                        미리보기
                      </Button>
                      <Button variant="ghost" onClick={() => removeFile(item.id)}>
                        삭제
                      </Button>
                    </div>
                  </div>
                ))}
                {files.length === 0 && <p className="text-sm text-slate-500">이미지를 추가해 주세요.</p>}
              </div>
            </div>
          </div>
          <div className="space-y-4">
            <div className="bg-slate-900 rounded-lg p-4">
              <h2 className="font-semibold mb-3">실시간 미리보기</h2>
              <div className="h-64 bg-slate-800 rounded-md flex items-center justify-center overflow-hidden">
                {previewUrl ? (
                  <img src={previewUrl} alt="preview" className="max-h-full max-w-full object-contain" />
                ) : (
                  <p className="text-slate-500 text-sm">미리보기를 생성하세요.</p>
                )}
              </div>
            </div>
            <div className="bg-slate-900 rounded-lg p-4 space-y-3">
              <h2 className="font-semibold">옵션</h2>
              <OptionSlider
                label="리사이즈 최소"
                value={options.resizeMin}
                min={0.5}
                max={1.0}
                step={0.01}
                onChange={(v) => updateOption('resizeMin', v)}
              />
              <OptionSlider
                label="리사이즈 최대"
                value={options.resizeMax}
                min={1.0}
                max={2.0}
                step={0.01}
                onChange={(v) => updateOption('resizeMax', v)}
              />
              <OptionSlider
                label="회전 최대 (deg)"
                value={options.rotateMaxDeg}
                min={0}
                max={10}
                step={0.5}
                onChange={(v) => updateOption('rotateMaxDeg', v)}
              />
              <OptionSlider
                label="밝기 범위"
                value={options.brightnessRange}
                min={0}
                max={20}
                step={0.5}
                onChange={(v) => updateOption('brightnessRange', v)}
              />
              <OptionSlider
                label="대비 범위"
                value={options.contrastRange}
                min={0}
                max={20}
                step={0.5}
                onChange={(v) => updateOption('contrastRange', v)}
              />
              <OptionSlider
                label="노이즈 시그마"
                value={options.noiseSigma}
                min={0}
                max={5}
                step={0.1}
                onChange={(v) => updateOption('noiseSigma', v)}
              />
              <OptionSlider
                label="JPEG 품질"
                value={options.jpegQuality}
                min={50}
                max={100}
                step={1}
                onChange={(v) => updateOption('jpegQuality', v)}
              />
              <OptionSlider
                label="WEBP 품질"
                value={options.webpQuality}
                min={50}
                max={100}
                step={1}
                onChange={(v) => updateOption('webpQuality', v)}
              />
              <div className="flex items-center gap-2">
                <input
                  id="stripExif"
                  type="checkbox"
                  checked={options.stripExif}
                  onChange={(e) => updateOption('stripExif', e.target.checked)}
                />
                <label htmlFor="stripExif" className="text-sm text-slate-200">
                  EXIF 제거
                </label>
              </div>
              <div className="flex flex-col gap-2">
                <label className="text-sm text-slate-200">출력 경로</label>
                <input
                  className="bg-slate-800 border border-slate-700 rounded px-3 py-2 text-sm text-slate-100"
                  placeholder="예: C:\\Users\\me\\Pictures"
                  value={outputDir}
                  onChange={(e) => setOutputDir(e.target.value)}
                />
              </div>
            </div>
          </div>
        </section>
      </main>
    </React.Fragment>
  );
};

interface OptionSliderProps {
  label: string;
  value: number;
  min: number;
  max: number;
  step: number;
  onChange: (value: number) => void;
}

const OptionSlider: React.FC<OptionSliderProps> = ({ label, value, min, max, step, onChange }) => {
  return (
    <div className="flex flex-col gap-1">
      <div className="flex items-center justify-between text-sm">
        <span>{label}</span>
        <span className="text-slate-400">{value.toFixed(2)}</span>
      </div>
      <input
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={(e) => onChange(parseFloat(e.target.value))}
      />
    </div>
  );
};
