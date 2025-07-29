'use client';

import React, { useState, useEffect } from 'react';
import { useDebounce } from 'use-debounce';
import { Play } from 'lucide-react';
import { JsonEditor } from './JsonEditor';
import { QueryBuilder } from './QueryBuilder';
import { ValidationResult } from './ValidationResult';

interface ValidationResponse {
  valid: boolean;
}

export const SiftValidator: React.FC = () => {
  const [jsonInput, setJsonInput] = useState<string>(`{
  "name": "Alice",
  "age": 30,
  "department": "Engineering",
  "skills": ["JavaScript", "Python", "Go"],
  "address": {
    "city": "San Francisco",
    "state": "CA"
  },
  "active": true
}`);

  const [mongoQuery, setMongoQuery] = useState<string>('{}');

  const [validationResult, setValidationResult] = useState<ValidationResponse | null>(null);
  const [validationError, setValidationError] = useState<string | null>(null);
  const [isValidating, setIsValidating] = useState(false);

  // Debounce the query and input to avoid too many API calls
  const [debouncedQuery] = useDebounce(mongoQuery, 500);
  const [debouncedInput] = useDebounce(jsonInput, 500);

  const validateQuery = async (query: string, input: string) => {
    try {
      // Parse both JSON strings to validate they're valid JSON
      const parsedInput = JSON.parse(input);
      const parsedQuery = JSON.parse(query);

      setIsValidating(true);
      setValidationError(null);

      const response = await fetch(`${process.env.NEXT_PUBLIC_SIFT_RS_API_URL}/validate`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify([{
          input: parsedInput,
          query: parsedQuery
        }])
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || `HTTP error! status: ${response.status}`);
      }

      const results = await response.json();
      setValidationResult(results[0]);
    } catch (error) {
      console.error('Validation error:', error);
      setValidationError(error instanceof Error ? error.message : 'Unknown error occurred');
      setValidationResult(null);
    } finally {
      setIsValidating(false);
    }
  };

  // Extract fields from JSON input for query builder reference
  const extractFields = (jsonStr: string): string[] => {
    try {
      const obj = JSON.parse(jsonStr);
      const fields: string[] = [];
      
      const extractFromObj = (obj: Record<string, unknown>, prefix: string = '') => {
        Object.keys(obj).forEach(key => {
          const fullKey = prefix ? `${prefix}.${key}` : key;
          fields.push(fullKey);
          
          if (obj[key] && typeof obj[key] === 'object' && !Array.isArray(obj[key])) {
            extractFromObj(obj[key] as Record<string, unknown>, fullKey);
          }
        });
      };
      
      extractFromObj(obj);
      return fields.sort();
    } catch {
      return [];
    }
  };

  const availableFields = extractFields(jsonInput);

  // Manual validation function
  const handleManualValidation = () => {
    if (mongoQuery.trim() && jsonInput.trim()) {
      validateQuery(mongoQuery, jsonInput);
    }
  };

  // Check if validation can be performed
  const canValidate = () => {
    try {
      const parsedQuery = JSON.parse(mongoQuery || '{}');
      const parsedInput = JSON.parse(jsonInput || '{}');
      return Object.keys(parsedQuery).length > 0 && Object.keys(parsedInput).length > 0;
    } catch {
      return false;
    }
  };

  // Validate whenever the debounced inputs change
  useEffect(() => {
    if (debouncedQuery.trim() && debouncedInput.trim()) {
      // Only validate if query is not empty {}
      try {
        const parsedQuery = JSON.parse(debouncedQuery);
        if (Object.keys(parsedQuery).length > 0) {
          validateQuery(debouncedQuery, debouncedInput);
        } else {
          // Clear validation result for empty query
          setValidationResult(null);
          setValidationError(null);
        }
      } catch {
        // Invalid JSON in query
        setValidationError('Invalid JSON in query');
        setValidationResult(null);
      }
    }
  }, [debouncedQuery, debouncedInput]);

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* JSON Input Section */}
        <div className="space-y-4">
          <div className="bg-white rounded-lg shadow-sm border">
            <div className="px-4 py-3 border-b bg-gray-50">
              <h2 className="text-lg font-semibold text-gray-900">JSON Input</h2>
              <p className="text-sm text-gray-600">Enter the JSON object to validate against</p>
            </div>
            <div className="p-4">
              <JsonEditor
                value={jsonInput}
                onChange={setJsonInput}
              />
            </div>
          </div>
        </div>

        {/* Query Builder Section */}
        <div className="space-y-4">
          <div className="bg-white rounded-lg shadow-sm border">
            <div className="px-4 py-3 border-b bg-gray-50">
              <h2 className="text-lg font-semibold text-gray-900">MongoDB Query Builder</h2>
              <p className="text-sm text-gray-600">Build your query using available fields</p>
            </div>
            <div className="p-4">
              <QueryBuilder
                value={mongoQuery}
                onChange={setMongoQuery}
                availableFields={availableFields}
              />
            </div>
          </div>
        </div>
      </div>

      {/* Manual Validation Button */}
      <div className="flex justify-center">
        <button
          onClick={handleManualValidation}
          disabled={!canValidate() || isValidating}
          className={`px-6 py-3 rounded-lg font-medium transition-colors flex items-center space-x-2 ${
            canValidate() && !isValidating
              ? 'bg-blue-600 text-white hover:bg-blue-700'
              : 'bg-gray-300 text-gray-500 cursor-not-allowed'
          }`}
        >
          <Play className="w-5 h-5" />
          <span>{isValidating ? 'Validating...' : 'Validate Query'}</span>
        </button>
      </div>

      {/* Validation Result Section */}
      <ValidationResult
        result={validationResult}
        error={validationError}
        isLoading={isValidating}
      />
    </div>
  );
};
