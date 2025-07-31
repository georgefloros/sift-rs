'use client';

import React, { useState, useCallback, useEffect } from 'react';
import Select from 'react-select';
import { Plus, Trash2, X, Undo2, Redo2, Edit3 } from 'lucide-react';

interface QueryBuilderProps {
  value: string;
  onChange: (value: string) => void;
  availableFields: string[];
}

interface FieldCondition {
  id: string;
  field: string;
  operator: string;
  value: unknown;
  type: 'field';
  group: 'and' | 'or';
}

interface WhereCondition {
  id: string;
  expression: string;
  type: 'where';
}

type QueryCondition = FieldCondition | WhereCondition;

interface ParsedQuery {
  $and?: Record<string, unknown>[];
  $or?: Record<string, unknown>[];
  [key: string]: unknown;
}

export const QueryBuilder: React.FC<QueryBuilderProps> = ({
  value,
  onChange,
  availableFields,
}) => {
  const [conditions, setConditions] = useState<QueryCondition[]>([]);
  const [currentCondition, setCurrentCondition] = useState<FieldCondition | WhereCondition | null>(null);
  const [currentWhere, setCurrentWhere] = useState<string>('');
  const [isAddingCondition, setIsAddingCondition] = useState(false);
  const [hasOrConditions, setHasOrConditions] = useState<boolean>(false);

  // Query history for undo/redo functionality
  const [queryHistory, setQueryHistory] = useState<string[]>(['{}']);
  const [historyIndex, setHistoryIndex] = useState(0);

  // Edit condition state
  const [editingConditionId, setEditingConditionId] = useState<string | null>(null);
  const [editingCondition, setEditingCondition] = useState<FieldCondition | WhereCondition | null>(null);
  const [editingWhere, setEditingWhere] = useState<string>('');

  // Flag to prevent circular updates during undo/redo
  const [isUndoRedoOperation, setIsUndoRedoOperation] = useState(false);
  
  // Flag to prevent circular updates when value is updated externally (e.g., from ChatInterface)
  const [isExternalUpdate, setIsExternalUpdate] = useState(false);

  const operators = [
    { value: '$eq', label: 'Equals', description: 'field equals value' },
    { value: '$ne', label: 'Not Equal', description: 'field does not equal value' },
    { value: '$gt', label: 'Greater Than', description: 'field > value' },
    { value: '$gte', label: 'Greater Than or Equal', description: 'field >= value' },
    { value: '$lt', label: 'Less Than', description: 'field < value' },
    { value: '$lte', label: 'Less Than or Equal', description: 'field <= value' },
    { value: '$in', label: 'In Array', description: 'field value is in array' },
    { value: '$nin', label: 'Not In Array', description: 'field value is not in array' },
    { value: '$exists', label: 'Exists', description: 'field exists' },
    { value: '$regex', label: 'Regular Expression', description: 'field matches regex' },
    { value: '$size', label: 'Array Size', description: 'array field has specific size' },
    { value: '$type', label: 'BSON Type', description: 'field is of specific BSON type' },
  ];
  // Parse existing query into conditions
  useEffect(() => {
    // Skip parsing if this is an undo/redo operation to prevent circular updates
    if (isUndoRedoOperation) {
      console.log('🔍 Skipping parse effect during undo/redo operation');
      return;
    }

    console.log('🔍 Parse effect triggered, value:', value);
    
    // Mark this as an external update to prevent rebuilding
    setIsExternalUpdate(true);
    try {
      const parsed = JSON.parse(value) as ParsedQuery;
      const newConditions: QueryCondition[] = [];
      let hasOr = false;

      // Handle structured format with recursive parsing for nested operators
      const parseRecursively = (conditions: Record<string, unknown>[], group: 'and' | 'or') => {
        conditions.forEach((condition: Record<string, unknown>) => {
          // Check if this condition contains nested logical operators
          if (condition.$and && Array.isArray(condition.$and)) {
            parseRecursively(condition.$and, 'and');
          } else if (condition.$or && Array.isArray(condition.$or)) {
            hasOr = true;
            parseRecursively(condition.$or, 'or');
          } else {
            const conditionObj = parseConditionFromQuery(condition, group);
            if (conditionObj) newConditions.push(conditionObj);
          }
        });
      };

      if (parsed.$and && Array.isArray(parsed.$and)) {
        parseRecursively(parsed.$and, 'and');
      }

      if (parsed.$or && Array.isArray(parsed.$or)) {
        hasOr = true;
        parseRecursively(parsed.$or, 'or');
      }

      // Handle legacy format for backward compatibility
      if (!parsed.$and && !parsed.$or) {
        Object.entries(parsed).forEach(([field, condition]) => {
          const conditionObj = parseConditionFromQuery({ [field]: condition }, 'and');
          if (conditionObj) newConditions.push(conditionObj);
        });
      }

      console.log('🔄 Updating conditions from parse effect, new length:', newConditions.length);
      setConditions(newConditions);
      setHasOrConditions(hasOr);
      
      // Reset external update flag after updating conditions
      setTimeout(() => setIsExternalUpdate(false), 0);
    } catch {
      console.log('🔍 Parse error, clearing conditions');
      setConditions([]);
      setHasOrConditions(false);
      
      // Reset external update flag even on parse error
      setTimeout(() => setIsExternalUpdate(false), 0);
    }
  }, [value, isUndoRedoOperation]);

  const parseConditionFromQuery = (condition: Record<string, unknown>, group: 'and' | 'or'): QueryCondition | null => {
    if (condition.$where) {
      return {
        id: `where-${Math.random()}`,
        expression: String(condition.$where),
        type: 'where',
      };
    }

    const [field, value] = Object.entries(condition)[0] as [string, unknown];
    if (field === '$where') {
      return {
        id: `where-${Math.random()}`,
        expression: String(value),
        type: 'where',
      };
    }

    // Handle nested logical operators (due to our nesting rule)
    if (field === '$and' && Array.isArray(value)) {
      // This is a nested $and inside another operator, treat as and group
      return null; // Will be handled by the recursive parsing
    }
    
    if (field === '$or' && Array.isArray(value)) {
      // This is a nested $or inside another operator, treat as or group
      return null; // Will be handled by the recursive parsing
    }

    if (typeof value === 'object' && value !== null) {
      const [operator, operatorValue] = Object.entries(value as Record<string, unknown>)[0] as [string, unknown];
      return {
        id: `${field}-${operator}-${Math.random()}`,
        field,
        operator,
        value: operatorValue,
        type: 'field',
        group,
      };
    } else {
      return {
        id: `${field}-$eq-${Math.random()}`,
        field,
        operator: '$eq',
        value,
        type: 'field',
        group,
      };
    }
  };

  const applyCondition = (group: 'and' | 'or' = 'and') => {
    if ((currentCondition && currentCondition.type === 'field' && currentCondition.field && currentCondition.operator && currentCondition.value) ||
      (currentCondition && currentCondition.type === 'where' && currentWhere.trim())) {
      const newCondition: QueryCondition = currentCondition.type === 'field'
        ? { ...currentCondition, id: `${currentCondition.field}-${currentCondition.operator}-${Date.now()}`, group }
        : { id: `${Date.now()}`, expression: currentWhere, type: 'where' as const };
      const newConditions = [...conditions, newCondition];
      setConditions(newConditions);
      // Update hasOrConditions if adding to OR group
      if (group === 'or') {
        setHasOrConditions(true);
      }
      buildQuery(newConditions, group === 'or' || hasOrConditions);
      setIsAddingCondition(false);
      setCurrentCondition(null);
      setCurrentWhere('');
    }
  };

  // Add query to history and trigger onChange
  const updateQueryWithHistory = useCallback((newQuery: string) => {
    // Only add to history if query is different from current and not during undo/redo
    if (newQuery !== value && !isUndoRedoOperation) {
      const newHistory = queryHistory.slice(0, historyIndex + 1);
      newHistory.push(newQuery);
      setQueryHistory(newHistory);
      setHistoryIndex(newHistory.length - 1);
    }
    onChange(newQuery);
  }, [value, queryHistory, historyIndex, onChange, isUndoRedoOperation]);

  const buildQuery = useCallback((newConditions: QueryCondition[], hasOr: boolean) => {
    if (newConditions.length === 0) {
      updateQueryWithHistory('{}');
      return;
    }

    const andConditions: Record<string, unknown>[] = [];
    const orConditions: Record<string, unknown>[] = [];
    
    // Separate conditions by group
    newConditions.forEach((condition) => {
      if (condition.type === 'field') {
        const conditionObj = { [condition.field]: { [condition.operator]: condition.value } };
        if (condition.group === 'or') {
          orConditions.push(conditionObj);
        } else {
          andConditions.push(conditionObj);
        }
      } else if (condition.type === 'where') {
        // $where conditions go to $and by default since they don't have groups
        andConditions.push({ '$where': condition.expression });
      }
    });

    // Apply the rule: $and and $or cannot be both at top level
    // If both are present, nest one inside the other based on which has conditions first
    let query: Record<string, unknown>;
    
    if (andConditions.length > 0 && orConditions.length > 0) {
      // Both $and and $or are present - always nest $and inside $or
      const nestedAndCondition = { $and: andConditions };
      query = {
        $or: [...orConditions, nestedAndCondition]
      };
    } else if (andConditions.length > 0) {
      // Only $and conditions
      if (andConditions.length === 1) {
        query = andConditions[0];
      } else {
        query = { $and: andConditions };
      }
    } else if (orConditions.length > 0) {
      // Only $or conditions
      query = { $or: orConditions };
    } else {
      // No conditions
      query = {};
    }

    updateQueryWithHistory(JSON.stringify(query, null, 2));
  }, [updateQueryWithHistory]);

  // Parse query string into conditions (helper function)
  const parseQueryIntoConditions = useCallback((queryString: string) => {
    console.log('🔍 Parsing query into conditions:', queryString);
    try {
      const parsed = JSON.parse(queryString) as ParsedQuery;
      const newConditions: QueryCondition[] = [];
      let hasOr = false;

      // Handle structured format with recursive parsing for nested operators
      const parseRecursively = (conditions: Record<string, unknown>[], group: 'and' | 'or') => {
        conditions.forEach((condition: Record<string, unknown>) => {
          // Check if this condition contains nested logical operators
          if (condition.$and && Array.isArray(condition.$and)) {
            parseRecursively(condition.$and, 'and');
          } else if (condition.$or && Array.isArray(condition.$or)) {
            hasOr = true;
            parseRecursively(condition.$or, 'or');
          } else {
            const conditionObj = parseConditionFromQuery(condition, group);
            if (conditionObj) newConditions.push(conditionObj);
          }
        });
      };

      if (parsed.$and && Array.isArray(parsed.$and)) {
        parseRecursively(parsed.$and, 'and');
      }

      if (parsed.$or && Array.isArray(parsed.$or)) {
        hasOr = true;
        parseRecursively(parsed.$or, 'or');
      }

      // Handle legacy format for backward compatibility
      if (!parsed.$and && !parsed.$or) {
        Object.entries(parsed).forEach(([field, condition]) => {
          const conditionObj = parseConditionFromQuery({ [field]: condition }, 'and');
          if (conditionObj) newConditions.push(conditionObj);
        });
      }

      return { conditions: newConditions, hasOr };
    } catch {
      console.log('🔍 Parse error in parseQueryIntoConditions');
      return { conditions: [], hasOr: false };
    }
  }, []);

  // Undo functionality
  const handleUndo = useCallback(() => {
    if (historyIndex > 0) {
      setIsUndoRedoOperation(true);
      const newIndex = historyIndex - 1;
      const queryToRestore = queryHistory[newIndex];
      
      // Parse the query and update conditions immediately
      const { conditions: newConditions, hasOr } = parseQueryIntoConditions(queryToRestore);
      
      setHistoryIndex(newIndex);
      setConditions(newConditions);
      setHasOrConditions(hasOr);
      onChange(queryToRestore);
      
      // Reset flag after a brief delay to allow state updates to complete
      setTimeout(() => setIsUndoRedoOperation(false), 1);
    }
  }, [historyIndex, queryHistory, onChange, parseQueryIntoConditions]);

  // Redo functionality
  const handleRedo = useCallback(() => {
    if (historyIndex < queryHistory.length - 1) {
      setIsUndoRedoOperation(true);
      const newIndex = historyIndex + 1;
      const queryToRestore = queryHistory[newIndex];
      
      // Parse the query and update conditions immediately
      const { conditions: newConditions, hasOr } = parseQueryIntoConditions(queryToRestore);
      
      setHistoryIndex(newIndex);
      setConditions(newConditions);
      setHasOrConditions(hasOr);
      onChange(queryToRestore);
      
      // Reset flag after a brief delay to allow state updates to complete
      setTimeout(() => setIsUndoRedoOperation(false), 1);
    }
  }, [historyIndex, queryHistory, onChange, parseQueryIntoConditions]);


  // Enable keyboard shortcuts
  useKeyboardShortcuts(handleUndo, handleRedo);

  const parseValue = (valueStr: string): unknown => {
    if (!valueStr.trim()) return '';

    // Try to parse as JSON first
    try {
      return JSON.parse(valueStr);
    } catch {
      // If it fails, treat as string
      return valueStr;
    }
  };


  const removeCondition = (id: string) => {
    const newConditions = conditions.filter((c) => c.id !== id);
    const stillHasOr = newConditions.some(c => c.type === 'field' && c.group === 'or');
    setConditions(newConditions);
    setHasOrConditions(stillHasOr);
    buildQuery(newConditions, stillHasOr);
  };

  const startEditCondition = (condition: QueryCondition) => {
    setEditingConditionId(condition.id);
    setEditingCondition({ ...condition });
    if (condition.type === 'where') {
      setEditingWhere(condition.expression);
    } else {
      setEditingWhere('');
    }
  };

  const cancelEditCondition = () => {
    setEditingConditionId(null);
    setEditingCondition(null);
    setEditingWhere('');
  };

  const saveEditCondition = () => {
    if (!editingCondition || !editingConditionId) return;

    const updatedConditions = conditions.map(condition => {
      if (condition.id === editingConditionId) {
        if (editingCondition.type === 'where') {
          return { ...editingCondition, expression: editingWhere };
        } else {
          return editingCondition;
        }
      }
      return condition;
    });

    setConditions(updatedConditions);
    const stillHasOr = updatedConditions.some(c => c.type === 'field' && c.group === 'or');
    setHasOrConditions(stillHasOr);
    buildQuery(updatedConditions, stillHasOr);
    cancelEditCondition();
  };

  const clearAll = () => {
    setConditions([]);
    setHasOrConditions(false);
    onChange('{}');
  };

  // Rebuild query when conditions change (but not during undo/redo or external updates)
  useEffect(() => {
    if (conditions.length > 0 && !isUndoRedoOperation && !isExternalUpdate) {
      console.log('🔧 Rebuilding query from conditions change');
      buildQuery(conditions, hasOrConditions);
    }
  }, [buildQuery, conditions, hasOrConditions, isUndoRedoOperation]);

  const formatValueDisplay = (value: unknown): string => {
    if (typeof value === 'string') return `"${value}"`;
    return JSON.stringify(value);
  };

  return (
    <div className="space-y-4" data-testid="query-builder">
      {/* Query Editor Toolbar */}
      <div className="flex items-center justify-between bg-white border border-gray-200 rounded-lg p-3">
        <div className="flex items-center space-x-2">
          <h3 className="text-sm font-semibold text-gray-900">Query Builder</h3>
          <div className="flex items-center space-x-1">
            <button
              onClick={handleUndo}
              disabled={historyIndex <= 0}
              data-testid="undo-button"
              className={`p-1.5 rounded transition-colors ${historyIndex <= 0
                ? 'text-gray-400 cursor-not-allowed'
                : 'text-gray-600 hover:text-blue-600 hover:bg-blue-50'
                }`}
              title="Undo (Ctrl+Z)"
            >
              <Undo2 className="w-4 h-4" />
            </button>
            <button
              onClick={handleRedo}
              disabled={historyIndex >= queryHistory.length - 1}
              data-testid="redo-button"
              className={`p-1.5 rounded transition-colors ${historyIndex >= queryHistory.length - 1
                ? 'text-gray-400 cursor-not-allowed'
                : 'text-gray-600 hover:text-blue-600 hover:bg-blue-50'
                }`}
              title="Redo (Ctrl+Y)"
            >
              <Redo2 className="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>
      {/* Current Conditions */}
      {conditions.length > 0 && (
        <div className="bg-gray-50 border border-gray-200 rounded-md p-3">
          <div className="flex justify-between items-center mb-3">
            <p className="text-sm font-semibold text-gray-900">Current Conditions:</p>
            <button
              onClick={clearAll}
              className="text-xs bg-red-100 text-red-700 px-2 py-1 rounded hover:bg-red-200 transition-colors"
            >
              Clear All
            </button>
          </div>
          <div className="space-y-2">
            {conditions.map((condition) => (
              <div
                key={condition.id}
                data-testid={`condition-row-${condition.id}`}
                className="flex items-center justify-between bg-white border rounded p-2"
              >
                <div className="flex items-center space-x-2 text-sm">
                  {condition.type === 'field' && (
                    <span className={`px-2 py-1 text-xs rounded font-bold ${condition.group === 'or' ? 'bg-orange-100 text-orange-800' : 'bg-blue-100 text-blue-800'
                      }`}>
                      {condition.group.toUpperCase()}
                    </span>
                  )}
                  {condition.type === 'where' && (
                    <span className="px-2 py-1 text-xs rounded font-bold bg-red-100 text-red-800">
                      TOP-LEVEL
                    </span>
                  )}
                  {condition.type === 'field' ? (
                    <>
                      <span className="font-mono text-blue-600">{condition.field}</span>
                      <span className="text-green-600">{condition.operator}</span>
                      <span className="text-purple-600">{formatValueDisplay(condition.value)}</span>
                    </>
                  ) : (
                    <>
                      <span className="font-mono text-red-600">$where</span>
                      <span className="text-purple-600">{condition.expression}</span>
                    </>
                  )}
                </div>
                <div className="flex items-center space-x-1">
                  <button
                    onClick={() => startEditCondition(condition)}
                    data-testid={`edit-condition-${condition.id}`}
                    className="text-blue-500 hover:text-blue-700 transition-colors"
                    title="Edit condition"
                  >
                    <Edit3 className="w-4 h-4" />
                  </button>
                  <button
                    onClick={() => removeCondition(condition.id)}
                    data-testid={`delete-condition-${condition.id}`}
                    className="text-red-500 hover:text-red-700 transition-colors"
                    title="Delete condition"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Edit Condition Form */}
      {editingConditionId && editingCondition && (
        <div className="bg-white border border-blue-200 rounded-lg p-4 space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-semibold text-gray-900">Edit Condition</h3>
            <button
              onClick={cancelEditCondition}
              className="text-gray-500 hover:text-gray-700"
              title="Cancel edit"
            >
              <X className="w-5 h-5" />
            </button>
          </div>

          {/* Field Condition Edit Form */}
          {editingCondition.type === 'field' && (
            <>
              {/* Select Field */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Select Field:
                </label>
                <Select
                  inputId="field-selection-dropdown-edit"
                  classNamePrefix="field-selection-dropdown-edit"
                  options={availableFields.map(f => ({ value: f, label: f }))}
                  onChange={(option) => option && setEditingCondition({ ...editingCondition, field: option.value })}
                  placeholder="Choose a field..."
                  value={editingCondition.field ? { value: editingCondition.field, label: editingCondition.field } : null}
                  isSearchable
                />
              </div>

              {/* Select Operator */}
              {editingCondition.field && (
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Select Operator:
                  </label>
                  <Select
                    inputId="operator-selection-dropdown-edit"
                    classNamePrefix="operator-selection-dropdown-edit"
                    options={operators.map(o => ({ value: o.value, label: `${o.value} - ${o.description}` }))}
                    onChange={(option) => option && setEditingCondition({ ...editingCondition, operator: option.value })}
                    placeholder="Choose an operator..."
                    value={editingCondition.operator ? { value: editingCondition.operator, label: `${editingCondition.operator} - ${operators.find(o => o.value === editingCondition.operator)?.description || ''}` } : null}
                    isSearchable
                  />
                </div>
              )}

              {/* Enter Value */}
              {editingCondition.operator && (
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Enter Value:
                  </label>
                  <input
                    data-testid="value-input"
                    type="text"
                    value={String(editingCondition.value || '')}
                    onChange={(e) => setEditingCondition({ ...editingCondition, value: parseValue(e.target.value) })}
                    placeholder='e.g., &quot;text&quot;, 25, [1, 2, 3], true'
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  />
                  <p className="text-xs text-gray-500 mt-1">
                    Tip: Use JSON format for arrays, objects, or boolean values
                  </p>
                </div>
              )}

              {/* Group Selection for Field Conditions */}
              {editingCondition.field && editingCondition.operator && editingCondition.value && (
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Logical Group:
                  </label>
                  <div className="flex space-x-4">
                    <button
                      onClick={() => setEditingCondition({ ...editingCondition, group: 'and' })}
                      className={`px-4 py-2 rounded text-sm font-medium transition-colors ${editingCondition.group === 'and'
                        ? 'bg-blue-600 text-white'
                        : 'bg-blue-100 text-blue-800 hover:bg-blue-200'
                        }`}
                    >
                      AND Group
                    </button>
                    <button
                      onClick={() => setEditingCondition({ ...editingCondition, group: 'or' })}
                      className={`px-4 py-2 rounded text-sm font-medium transition-colors ${editingCondition.group === 'or'
                        ? 'bg-orange-600 text-white'
                        : 'bg-orange-100 text-orange-800 hover:bg-orange-200'
                        }`}
                    >
                      OR Group
                    </button>
                  </div>
                </div>
              )}
            </>
          )}

          {/* Where Expression Edit Form */}
          {editingCondition.type === 'where' && (
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Enter JavaScript Expression:
              </label>
              <input
                data-testid="where-expression-input"
                type="text"
                value={editingWhere}
                onChange={(e) => setEditingWhere(e.target.value)}
                placeholder='e.g., this.age > 25 && this.department === &quot;Engineering&quot;'
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              />
              <p className="text-xs text-gray-500 mt-1">
                Use JavaScript syntax. &quot;this&quot; refers to the document being evaluated.
              </p>
            </div>
          )}

          {/* Save/Cancel Buttons */}
          {((editingCondition.type === 'field' && editingCondition.field && editingCondition.operator && editingCondition.value) ||
            (editingCondition.type === 'where' && editingWhere.trim())) && (
              <div className="pt-4 space-y-3">
                <div className="flex space-x-2">
                  <button
                    onClick={saveEditCondition}
                    className="flex-1 bg-green-600 text-white py-2 px-4 rounded-lg hover:bg-green-700 transition-colors"
                  >
                    Save Changes
                  </button>
                  <button
                    onClick={cancelEditCondition}
                    className="flex-1 bg-gray-600 text-white py-2 px-4 rounded-lg hover:bg-gray-700 transition-colors"
                  >
                    Cancel
                  </button>
                </div>
              </div>
            )}
        </div>
      )}

      {/* Add Condition Form */}
      {!isAddingCondition && !editingConditionId ? (
        <button
          data-testid="add-new-condition-button"
          onClick={() => setIsAddingCondition(true)}
          className="w-full bg-green-100 border-2 border-dashed border-green-300 text-green-700 py-4 px-4 rounded-lg hover:bg-green-200 transition-colors flex items-center justify-center space-x-2"
        >
          <Plus className="w-5 h-5" />
          <span>Add New Condition</span>
        </button>
      ) : (
        <div className="bg-white border border-gray-200 rounded-lg p-4 space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-semibold text-gray-900">Add New Condition</h3>
            <button
              title='Add New Condition'
              onClick={() => {
                setIsAddingCondition(false);
                setCurrentCondition(null);
                setCurrentWhere('');
              }}
              className="text-gray-500 hover:text-gray-700"
            >
              <X className="w-5 h-5" />
            </button>
          </div>

          {/* Condition Type Selection */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Select Condition Type:
            </label>
            <div className="flex space-x-4">
              <button
                onClick={() => setCurrentCondition({ id: '', field: '', operator: '', value: '', type: 'field', group: 'and' })}
                data-testid="field-condition-button"
                className={`px-4 py-2 rounded text-sm font-medium transition-colors ${currentCondition?.type === 'field'
                  ? 'bg-blue-600 text-white'
                  : 'bg-blue-100 text-blue-800 hover:bg-blue-200'
                  }`}
              >
                Field Condition
              </button>
              <button
                onClick={() => setCurrentCondition({ id: '', expression: '', type: 'where' })}
                data-testid="where-expression-button"
                className={`px-4 py-2 rounded text-sm font-medium transition-colors ${currentCondition?.type === 'where'
                  ? 'bg-green-600 text-white'
                  : 'bg-green-100 text-green-800 hover:bg-green-200'
                  }`}
              >
                $where Expression
              </button>
            </div>
          </div>

          {/* Field Condition Form */}
          {currentCondition?.type === 'field' && (
            <>
              {/* Select Field */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Select Field:
                </label>
                <Select
                  inputId="field-selection-dropdown"
                  classNamePrefix="field-selection-dropdown"
                  options={availableFields.map(f => ({ value: f, label: f }))}
                  onChange={(option) => option && setCurrentCondition({ ...currentCondition, field: option.value })}
                  placeholder="Choose a field..."
                  value={currentCondition.field ? { value: currentCondition.field, label: currentCondition.field } : null}
                  isSearchable
                />
              </div>

              {/* Select Operator */}
              {currentCondition.field && (
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Select Operator:
                  </label>
                  <Select
                    inputId="operator-selection-dropdown"
                    classNamePrefix="operator-selection-dropdown"
                    options={operators.map(o => ({ value: o.value, label: `${o.value} - ${o.description}` }))}
                    onChange={(option) => option && setCurrentCondition({ ...currentCondition, operator: option.value })}
                    placeholder="Choose an operator..."
                    value={currentCondition.operator ? { value: currentCondition.operator, label: `${currentCondition.operator} - ${operators.find(o => o.value === currentCondition.operator)?.description || ''}` } : null}
                    isSearchable
                  />
                </div>
              )}

              {/* Enter Value */}
              {currentCondition.operator && (
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Enter Value:
                  </label>
                  <input
                    data-testid="value-input"
                    type="text"
                    value={String(currentCondition.value || '')}
                    onChange={(e) => setCurrentCondition({ ...currentCondition, value: parseValue(e.target.value) })}
                    placeholder='e.g., "text", 25, [1, 2, 3], true'
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  />
                  <p className="text-xs text-gray-500 mt-1">
                    Tip: Use JSON format for arrays, objects, or boolean values
                  </p>
                </div>
              )}
            </>
          )}

          {/* Where Expression Form */}
          {currentCondition?.type === 'where' && (
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Enter JavaScript Expression:
              </label>
              <input
                data-testid="where-expression-input"
                type="text"
                value={currentWhere}
                onChange={(e) => setCurrentWhere(e.target.value)}
                placeholder='e.g., this.age > 25 && this.department === "Engineering"'
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              />
              <p className="text-xs text-gray-500 mt-1">
                Use JavaScript syntax. &quot;this&quot; refers to the document being evaluated.
              </p>
            </div>
          )}

          {/* Add Condition Buttons with Group Selection */}
          {((currentCondition?.type === 'field' && currentCondition.field && currentCondition.operator && currentCondition.value) ||
            (currentCondition?.type === 'where' && currentWhere.trim())) && (
              <div className="pt-4 space-y-3">
                <div className="flex space-x-2">
                  <button
                    onClick={() => applyCondition('and')}
                    data-testid="add-to-and-group-button"
                    className="flex-1 bg-blue-600 text-white py-2 px-4 rounded-lg hover:bg-blue-700 transition-colors"
                  >
                    Add to AND Group
                  </button>
                  <button
                    onClick={() => applyCondition('or')}
                    data-testid="add-to-or-group-button"
                    className="flex-1 bg-orange-600 text-white py-2 px-4 rounded-lg hover:bg-orange-700 transition-colors"
                  >
                    Add to OR Group
                  </button>
                </div>
                <p className="text-xs text-gray-500 text-center">
                  Choose which logical group this condition belongs to
                </p>
              </div>
            )}
        </div>
      )}

      {/* Generated Query Preview */}
      {conditions.length > 0 && (
        <div className="bg-gray-900 text-gray-100 rounded-lg p-4">
          <p className="text-sm font-semibold mb-2">Generated MongoDB Query:</p>
          <pre className="text-sm overflow-x-auto">
            {JSON.stringify(
              JSON.parse(value.length > 0 ? value : '{}'),
              null,
              2
            )}
          </pre>
        </div>
      )}
    </div>
  );
};

// Add keyboard shortcuts support
const useKeyboardShortcuts = (handleUndo: () => void, handleRedo: () => void) => {
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 'z' && !e.shiftKey) {
        e.preventDefault();
        handleUndo();
      } else if (
        ((e.ctrlKey || e.metaKey) && e.key === 'y') ||
        ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'Z')
      ) {
        e.preventDefault();
        handleRedo();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleUndo, handleRedo]);
};

export default QueryBuilder; // Ensure this is the final line

