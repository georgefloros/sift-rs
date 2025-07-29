'use client';

import React from 'react';
import dynamic from 'next/dynamic';

// Dynamically import Monaco Editor to avoid SSR issues
const MonacoEditor = dynamic(() => import('@monaco-editor/react'), {
  ssr: false,
  loading: () => (
    <div className="h-64 bg-gray-100 rounded border flex items-center justify-center">
      <p className="text-gray-500">Loading editor...</p>
    </div>
  ),
});

interface JsonEditorProps {
  value: string;
  onChange: (value: string) => void;
  height?: string;
}

export const JsonEditor: React.FC<JsonEditorProps> = ({
  value,
  onChange,
  height = "300px"
}) => {
  const handleEditorChange = (value: string | undefined) => {
    onChange(value || '');
  };

  return (
    <div className="border rounded-md overflow-hidden">
      <MonacoEditor
        height={height}
        defaultLanguage="json"
        value={value}
        onChange={handleEditorChange}
        options={{
          minimap: { enabled: false },
          scrollBeyondLastLine: false,
          fontSize: 14,
          tabSize: 2,
          formatOnPaste: true,
          formatOnType: true,
          automaticLayout: true,
          wordWrap: 'on',
          lineNumbers: 'on',
          folding: true,
          bracketPairColorization: { enabled: true },
          suggest: {
            showKeywords: true,
            showSnippets: true,
          },
          quickSuggestions: {
            strings: true,
            comments: true,
            other: true,
          },
        }}
        theme="vs-light"
      />
    </div>
  );
};
