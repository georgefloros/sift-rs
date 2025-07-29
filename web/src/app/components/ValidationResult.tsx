'use client';

import React from 'react';
import { AlertTriangle, CheckCircle } from 'lucide-react';

interface ValidationResultProps {
  result: { valid: boolean } | null;
  error: string | null;
  isLoading: boolean;
}

export const ValidationResult: React.FC<ValidationResultProps> = ({
  result,
  error,
  isLoading,
}) => {
  if (isLoading) {
    return (
      <div className="bg-white rounded-lg shadow-sm border p-8">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
          <p className="text-gray-600 mt-2">Validating query...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-white rounded-lg shadow-sm border p-8">
        <div className="text-center">
          <AlertTriangle className="w-8 h-8 text-red-600 mx-auto" />
          <p className="text-red-600 mt-2 font-medium">Validation Error</p>
          <p className="text-red-500 text-sm mt-1">{error}</p>
        </div>
      </div>
    );
  }

  if (result) {
    return (
      <div className="bg-white rounded-lg shadow-sm border p-8">
        <div className="text-center">
          {result.valid ? (
            <>
              <CheckCircle className="w-16 h-16 text-green-600 mx-auto" />
              <p className="text-2xl font-bold text-green-700 mt-4">
                Validation Passed ✓
              </p>
              <p className="text-green-600 text-sm mt-2">
                The JSON object matches the MongoDB query
              </p>
            </>
          ) : (
            <>
              <AlertTriangle className="w-16 h-16 text-red-600 mx-auto" />
              <p className="text-2xl font-bold text-red-700 mt-4">
                Validation Failed ✗
              </p>
              <p className="text-red-600 text-sm mt-2">
                The JSON object does not match the MongoDB query
              </p>
            </>
          )}
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-sm border p-8">
      <div className="text-center">
        <div className="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto">
          <span className="text-2xl text-gray-400">?</span>
        </div>
        <p className="text-xl font-medium text-gray-700 mt-4">
          Ready to Validate
        </p>
        <p className="text-gray-500 text-sm mt-2">
          Build a MongoDB query using the wizard above to test validation
        </p>
      </div>
    </div>
  );
};

