import React from 'react';

export type UploadFile = {
  id: string;
  file: File;
};

export const useUploadList = () => {
  const [files, setFiles] = React.useState<UploadFile[]>([]);

  const addFiles = (incoming: File[]) => {
    setFiles((prev) => [
      ...prev,
      ...incoming.map((file) => ({ id: `${file.name}-${file.size}-${file.lastModified}-${crypto.randomUUID()}`, file })),
    ]);
  };

  const removeFile = (id: string) => setFiles((prev) => prev.filter((item) => item.id != id));

  const clearFiles = () => setFiles([]);

  return { files, addFiles, removeFile, clearFiles };
};
