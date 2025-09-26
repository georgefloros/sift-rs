import { Component, Prop, h, State, Event, EventEmitter, Watch, Method } from '@stencil/core';

// --- Interfaces ---
export interface FieldCondition {
  id: string;
  field: string;
  operator: string;
  value: unknown;
  type: 'field';
  group: 'and' | 'or';
}

export interface WhereCondition {
  id:string;
  expression: string;
  type: 'where';
}

export type QueryCondition = FieldCondition | WhereCondition;

interface ParsedQuery {
  $and?: Record<string, unknown>[];
  $or?: Record<string, unknown>[];
  [key: string]: unknown;
}


@Component({
  tag: 'query-builder-component',
  styleUrl: 'query-builder.css',
  shadow: true,
})
export class QueryBuilder {
  // --- Props ---
  @Prop({ mutable: true }) value: string = '{}';
  @Prop({ mutable: true }) availableFields: string[] = [];
  @Prop({ mutable: true }) jsonInput: object | any[] = null;
  @Prop() renderCondition: (condition: QueryCondition) => string;
  @Prop() localization: any;

  private get localizedOperators() {
    if (!this.localization?.operators) {
      return this.operators;
    }
    return this.operators.map(op => ({
      ...op,
      label: this.localization.operators[op.value] || op.label,
    }));
  }

  // --- State ---
  @State() conditions: QueryCondition[] = [];
  @State() isAddingCondition: boolean = false;
  @State() hasOrConditions: boolean = false;
  
  // Form States
  @State() currentConditionType: 'field' | 'where' | null = null;
  @State() currentField: string = '';
  @State() currentOperator: string = '';
  @State() currentValue: string = '';
  @State() currentWhereExpression: string = '';

  // Edit State
  @State() editingConditionId: string | null = null;
  @State() editingCondition: QueryCondition | null = null;

  // History State
  @State() queryHistory: string[] = ['{}'];
  @State() historyIndex: number = 0;
  private isUndoRedo: boolean = false;

  // --- Events ---
  @Event() queryChanged: EventEmitter<string>;
  @Event() historyUpdated: EventEmitter<{ canUndo: boolean; canRedo: boolean; }>;

  // --- Data ---
  private operators = [
    { value: '$eq', label: 'Equals' }, { value: '$ne', label: 'Not Equal' },
    { value: '$gt', label: 'Greater Than' }, { value: '$gte', label: 'Greater Than or Equal' },
    { value: '$lt', label: 'Less Than' }, { value: '$lte', label: 'Less Than or Equal' },
    { value: '$in', label: 'In Array' }, { value: '$nin', label: 'Not In Array' },
    { value: '$exists', label: 'Exists' }, { value: '$regex', label: 'Regular Expression' },
    { value: '$size', label: 'Array Size' }, { value: '$type', label: 'BSON Type' },
  ];

  // --- Lifecycle ---
  componentWillLoad() {
    this.parseJsonInput();
  }

  componentDidLoad() {
    this.emitHistoryUpdate();
  }

  // --- Watchers ---
  @Watch('value')
  onExternalValueChange(newValue: string) {
    // If the change is from undo/redo, don't re-parse.
    if (this.isUndoRedo) return;
    
    // If the new value is different from what's in the history, treat as external change
    if (newValue !== this.queryHistory[this.historyIndex]) {
      this.parseQueryToState(newValue);
      this.updateQueryWithHistory(newValue, true); // Add to history
    }
  }

  @Watch('jsonInput')
  onJsonInputChanged() {
    this.parseJsonInput();
  }

  // --- Public Methods ---
  @Method()
  async undo() {
    if (this.historyIndex > 0) {
      this.isUndoRedo = true;
      this.historyIndex--;
      const queryToRestore = this.queryHistory[this.historyIndex];
      this.value = queryToRestore;
      this.parseQueryToState(queryToRestore);
      this.queryChanged.emit(queryToRestore);
      this.emitHistoryUpdate();
      setTimeout(() => this.isUndoRedo = false, 0);
    }
  }

  @Method()
  async redo() {
    if (this.historyIndex < this.queryHistory.length - 1) {
      this.isUndoRedo = true;
      this.historyIndex++;
      const queryToRestore = this.queryHistory[this.historyIndex];
      this.value = queryToRestore;
      this.parseQueryToState(queryToRestore);
      this.queryChanged.emit(queryToRestore);
      this.emitHistoryUpdate();
      setTimeout(() => this.isUndoRedo = false, 0);
    }
  }

  // --- Private Methods ---
  private getNestedKeys(obj: object, prefix: string = ''): string[] {
    if (!obj) {
      return [];
    }

    return Object.keys(obj).flatMap(key => {
      const newKey = prefix ? `${prefix}.${key}` : key;
      const value = obj[key];

      if (typeof value === 'object' && value !== null && !Array.isArray(value)) {
        return [newKey, ...this.getNestedKeys(value, newKey)];
      } else if (Array.isArray(value) && value.length > 0 && typeof value[0] === 'object' && value[0] !== null) {
        return [newKey, ...this.getNestedKeys(value[0], newKey)];
      }
      
      return [newKey];
    });
  }

  private parseJsonInput() {
    if (!this.jsonInput) {
      this.availableFields = [];
      return;
    }
    
    let sampleObject: object;
    if (Array.isArray(this.jsonInput) && this.jsonInput.length > 0 && typeof this.jsonInput[0] === 'object' && this.jsonInput[0] !== null) {
        sampleObject = this.jsonInput[0];
    } else if (typeof this.jsonInput === 'object' && !Array.isArray(this.jsonInput)) {
        sampleObject = this.jsonInput as object;
    } else {
        this.availableFields = [];
        return;
    }

    const keys = this.getNestedKeys(sampleObject);
    this.availableFields = [...new Set(keys)].sort();
  }

  private emitHistoryUpdate() {
    this.historyUpdated.emit({
      canUndo: this.historyIndex > 0,
      canRedo: this.historyIndex < this.queryHistory.length - 1,
    });
  }

  private resetAddForm() {
    this.isAddingCondition = false;
    this.currentConditionType = null;
    this.currentField = '';
    this.currentOperator = '';
    this.currentValue = '';
    this.currentWhereExpression = '';
  }

  private parseInputValue = (valueStr: string): unknown => {
    const trimmed = valueStr.trim();
    if (!trimmed) return '';
    try { return JSON.parse(trimmed); }
    catch { return trimmed; }
  }

  private applyCondition(group: 'and' | 'or') {
    let newCondition: QueryCondition | null = null;
    if (this.currentConditionType === 'field' && this.currentField && this.currentOperator) {
      newCondition = { id: `${this.currentField}-${this.currentOperator}-${Date.now()}`, type: 'field', field: this.currentField, operator: this.currentOperator, value: this.parseInputValue(this.currentValue), group: group };
    } else if (this.currentConditionType === 'where' && this.currentWhereExpression.trim()) {
      newCondition = { id: `where-${Date.now()}`, type: 'where', expression: this.currentWhereExpression };
    }

    if (newCondition) {
      this.conditions = [...this.conditions, newCondition];
      if (newCondition.type === 'field' && newCondition.group === 'or') this.hasOrConditions = true;
      this.buildQuery();
      this.resetAddForm();
    }
  }

  private removeCondition(id: string) {
    this.conditions = this.conditions.filter(c => c.id !== id);
    this.hasOrConditions = this.conditions.some(c => c.type === 'field' && c.group === 'or');
    this.buildQuery();
  }

  private startEditCondition(condition: QueryCondition) {
    this.editingConditionId = condition.id;
    this.editingCondition = { ...condition };
  }

  private cancelEditCondition() {
    this.editingConditionId = null;
    this.editingCondition = null;
  }

  private saveEditCondition() {
    if (!this.editingConditionId || !this.editingCondition) return;
    this.conditions = this.conditions.map(c => c.id === this.editingConditionId ? this.editingCondition : c);
    this.hasOrConditions = this.conditions.some(c => c.type === 'field' && c.group === 'or');
    this.buildQuery();
    this.cancelEditCondition();
  }

  private buildQuery() {
    let query: Record<string, unknown>;
    if (this.conditions.length === 0) { query = {}; }
    else {
      const and: Record<string, unknown>[] = [], or: Record<string, unknown>[] = [];
      this.conditions.forEach(c => {
        if (c.type === 'field') {
          const obj = { [c.field]: { [c.operator]: c.value } };
          if (c.group === 'or') or.push(obj); else and.push(obj);
        } else if (c.type === 'where') { and.push({ '$where': c.expression }); }
      });
      if (and.length > 0 && or.length > 0) { query = { $or: [...or, { $and: and }] }; }
      else if (and.length > 0) { query = and.length === 1 ? and[0] : { $and: and }; }
      else if (or.length > 0) { query = or.length === 1 ? or[0] : { $or: or }; }
      else { query = {}; }
    }
    this.updateQueryWithHistory(JSON.stringify(query, null, 2));
  }

  private updateQueryWithHistory(newQuery: string, force: boolean = false) {
    if (newQuery !== this.value || force) {
      const newHistory = this.queryHistory.slice(0, this.historyIndex + 1);
      newHistory.push(newQuery);
      this.queryHistory = newHistory;
      this.historyIndex = newHistory.length - 1;
      this.value = newQuery;
      this.queryChanged.emit(this.value);
      this.emitHistoryUpdate();
    }
  }

  private clearAllConditions() {
    this.conditions = [];
    this.hasOrConditions = false;
    this.buildQuery();
  }

  private isAddFormReady(): boolean {
    return !!((this.currentConditionType === 'field' && this.currentField && this.currentOperator) || (this.currentConditionType === 'where' && this.currentWhereExpression.trim()));
  }

  private parseQueryToState(queryString: string) {
    try {
      const parsed = JSON.parse(queryString) as ParsedQuery;
      const newConditions: QueryCondition[] = [];
      let hasOr = false;
      const parseCondition = (c: Record<string, unknown>, g: 'and' | 'or'): QueryCondition | null => {
        if (c.$where) return { id: `where-${Math.random()}`, expression: String(c.$where), type: 'where' };
        const [f, v] = Object.entries(c)[0];
        if (typeof v === 'object' && v !== null && !Array.isArray(v)) {
          const [op, opV] = Object.entries(v as Record<string, unknown>)[0];
          return { id: `${f}-${op}-${Math.random()}`, field: f, operator: op, value: opV, type: 'field', group: g };
        }
        return { id: `${f}-$eq-${Math.random()}`, field: f, operator: '$eq', value: v, type: 'field', group: g };
      };
      const parseRec = (conds: Record<string, unknown>[], group: 'and' | 'or') => {
        conds.forEach(c => {
          if (c.$and && Array.isArray(c.$and)) parseRec(c.$and, 'and');
          else if (c.$or && Array.isArray(c.$or)) { hasOr = true; parseRec(c.$or, 'or'); }
          else { const cond = parseCondition(c, group); if (cond) newConditions.push(cond); }
        });
      };
      if (parsed.$and && Array.isArray(parsed.$and)) parseRec(parsed.$and, 'and');
      if (parsed.$or && Array.isArray(parsed.$or)) { hasOr = true; parseRec(parsed.$or, 'or'); }
      if (!parsed.$and && !parsed.$or && Object.keys(parsed).length > 0) {
        Object.entries(parsed).forEach(([f, c]) => { const o = parseCondition({ [f]: c }, 'and'); if (o) newConditions.push(o); });
      }
      this.conditions = newConditions;
      this.hasOrConditions = hasOr;
    } catch { this.conditions = []; this.hasOrConditions = false; }
  }

  // --- Render Methods ---
  private formatValueDisplay = (value: unknown): string => typeof value === 'string' ? `"${value}"` : JSON.stringify(value);

  render() {
    return (
      <div class="space-y-4" part="root-container">
        <div class="bg-white rounded-lg shadow-sm border" part="main-panel">
          <div class="px-4 py-3 border-b bg-gray-50" part="header-section">
            <slot name="header">
              <h2 class="text-lg font-semibold text-gray-900" part="header-title">MongoDB Query Builder</h2>
              <p class="text-sm text-gray-600" part="header-description">Build your query using available fields</p>
            </slot>
          </div>
          <div class="p-4" part="content-wrapper">
            <div class="space-y-4" data-testid="query-builder" part="query-builder-container">
              <div class="flex items-center justify-between bg-white border border-gray-200 rounded-lg p-3" part="toolbar">
                <div class="flex items-center space-x-2" part="toolbar-left">
                  <h3 class="text-sm font-semibold text-gray-900" part="toolbar-title">Query Builder</h3>
                  <div class="flex items-center space-x-1" part="history-controls">
                    <button 
                      data-testid="undo-button" 
                      class="p-1.5 rounded transition-colors text-gray-600 hover:text-blue-600 hover:bg-blue-50" 
                      part="undo-button"
                      title="Undo (Ctrl+Z)"
                      onClick={() => this.undo()}
                      disabled={this.historyIndex <= 0}
                    >
                      <slot name="undo-icon">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-undo2 lucide-undo-2 w-4 h-4" aria-hidden="true" part="undo-icon-svg">
                          <path d="M9 14 4 9l5-5"></path>
                          <path d="M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5a5.5 5.5 0 0 1-5.5 5.5H11"></path>
                        </svg>
                      </slot>
                    </button>
                    <button 
                      data-testid="redo-button" 
                      class="p-1.5 rounded transition-colors text-gray-400 cursor-not-allowed" 
                      part="redo-button"
                      title="Redo (Ctrl+Y)"
                      onClick={() => this.redo()}
                      disabled={this.historyIndex >= this.queryHistory.length - 1}
                    >
                      <slot name="redo-icon">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-redo2 lucide-redo-2 w-4 h-4" aria-hidden="true" part="redo-icon-svg">
                          <path d="m15 14 5-5-5-5"></path>
                          <path d="M20 9H9.5A5.5 5.5 0 0 0 4 14.5A5.5 5.5 0 0 0 9.5 20H13"></path>
                        </svg>
                      </slot>
                    </button>
                  </div>
                </div>
              </div>

              {this.conditions.length > 0 && (
                <div class="bg-gray-50 border border-gray-200 rounded-md p-3" part="conditions-container">
                  <div class="flex justify-between items-center mb-3" part="conditions-header">
                    <p class="text-sm font-semibold text-gray-900" part="conditions-title">Current Conditions:</p>
                    <button 
                      class="text-xs bg-red-100 text-red-700 px-2 py-1 rounded hover:bg-red-200 transition-colors" 
                      part="clear-all-button"
                      onClick={() => this.clearAllConditions()}
                    >
                      <slot name="clear-all-text">Clear All</slot>
                    </button>
                  </div>
                  <div class="space-y-2" part="conditions-list">
                    {this.conditions.map(c => {
                      return (
                        <div 
                          key={c.id} 
                          data-testid={`condition-row-${c.type === 'field' ? `${c.field}-${c.operator}` : c.type}-${c.id.split('-').pop() || c.id}`} 
                          class="flex items-center justify-between bg-white border rounded p-2"
                          part="condition-row"
                        >
                          <div class="flex items-center space-x-2 text-sm" part="condition-content">
                            {c.type === 'field' && (
                              <span class={`px-2 py-1 text-xs rounded font-bold bg-blue-100 text-blue-800`} part="condition-group-badge">
                                {c.group.toUpperCase()}
                              </span>
                            )}
                            {c.type === 'where' && (
                              <span class="px-2 py-1 text-xs rounded font-bold bg-red-100 text-red-800" part="condition-where-badge">
                                TOP-LEVEL
                              </span>
                            )}
                            <span class="font-mono text-blue-600" part="condition-field">
                              {c.type === 'field' ? c.field : '$where'}
                            </span>
                            {c.type === 'field' && (
                              <span class="text-green-600" part="condition-operator">{c.operator}</span>
                            )}
                            <span class="text-purple-600" part="condition-value">
                              {c.type === 'field' ? this.formatValueDisplay(c.value) : c.expression}
                            </span>
                          </div>
                          <div class="flex items-center space-x-1" part="condition-actions">
                            <button 
                              data-testid={`edit-condition-${c.type === 'field' ? `${c.field}-${c.operator}` : c.type}-${c.id.split('-').pop() || c.id}`} 
                              class="text-blue-500 hover:text-blue-700 transition-colors" 
                              part="edit-button"
                              title="Edit condition"
                              onClick={() => this.startEditCondition(c)}
                            >
                              <slot name="edit-icon">
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-pen-line w-4 h-4" aria-hidden="true" part="edit-icon-svg">
                                  <path d="M13 21h8"></path>
                                  <path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"></path>
                                </svg>
                              </slot>
                            </button>
                            <button 
                              data-testid={`delete-condition-${c.type === 'field' ? `${c.field}-${c.operator}` : c.type}-${c.id.split('-').pop() || c.id}`} 
                              class="text-red-500 hover:text-red-700 transition-colors" 
                              part="delete-button"
                              title="Delete condition"
                              onClick={() => this.removeCondition(c.id)}
                            >
                              <slot name="delete-icon">
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-trash2 lucide-trash-2 w-4 h-4" aria-hidden="true" part="delete-icon-svg">
                                  <path d="M10 11v6"></path>
                                  <path d="M14 11v6"></path>
                                  <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"></path>
                                  <path d="M3 6h18"></path>
                                  <path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
                                </svg>
                              </slot>
                            </button>
                          </div>
                        </div>
                      );
                    })}
                  </div>
                </div>
              )}

              <div class="bg-white border border-gray-200 rounded-lg p-4 space-y-4" part="add-condition-container">
                <div class="flex items-center justify-between" part="add-condition-header">
                  <h3 class="text-lg font-semibold text-gray-900" part="add-condition-title">
                    <slot name="add-condition-title">Add New Condition</slot>
                  </h3>
                  <button 
                    title="Close" 
                    class="text-gray-500 hover:text-gray-700"
                    part="close-add-button"
                    onClick={() => this.resetAddForm()}
                  >
                    <slot name="close-icon">
                      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-x w-5 h-5" aria-hidden="true" part="close-icon-svg">
                        <path d="M18 6 6 18"></path>
                        <path d="m6 6 12 12"></path>
                      </svg>
                    </slot>
                  </button>
                </div>
                
                <div part="condition-type-selector">
                  <label class="block text-sm font-medium text-gray-700 mb-2" part="condition-type-label">
                    <slot name="condition-type-label">Select Condition Type:</slot>
                  </label>
                  <div class="flex space-x-4" part="condition-type-buttons">
                    <button 
                      data-testid="field-condition-button" 
                      class={`px-4 py-2 rounded text-sm font-medium transition-colors ${this.currentConditionType === 'field' ? 'bg-blue-600 text-white' : 'bg-green-100 text-green-800 hover:bg-green-200'}`}
                      part="field-condition-button"
                      onClick={() => this.currentConditionType = 'field'}
                    >
                      <slot name="field-condition-text">Field Condition</slot>
                    </button>
                    <button 
                      data-testid="where-expression-button" 
                      class={`px-4 py-2 rounded text-sm font-medium transition-colors ${this.currentConditionType === 'where' ? 'bg-blue-600 text-white' : 'bg-green-100 text-green-800 hover:bg-green-200'}`}
                      part="where-expression-button"
                      onClick={() => this.currentConditionType = 'where'}
                    >
                      <slot name="where-expression-text">$where Expression</slot>
                    </button>
                  </div>
                </div>
                
                {this.currentConditionType === 'field' && (
                  <div part="field-selector">
                    <label class="block text-sm font-medium text-gray-700 mb-2" part="field-label">
                      <slot name="field-label">Select Field:</slot>
                    </label>
                    <div class="css-b62m3t-container" part="field-dropdown-container">
                      <span id="react-select-7-live-region" class="css-1f43avz-a11yText-A11yText"></span>
                      <span aria-live="polite" aria-atomic="false" aria-relevant="additions text" role="log" class="css-1f43avz-a11yText-A11yText"></span>
                      <div class="field-selection-dropdown__control css-13cymwt-control">
                        <div class="field-selection-dropdown__value-container field-selection-dropdown__value-container--has-value css-hlgwow">
                          <div class="field-selection-dropdown__single-value css-1dimb5e-singleValue">
                            {this.currentField || 'Select Field...'}
                          </div>
                          <div class="field-selection-dropdown__input-container css-19bb58m" data-value="">
                            <select 
                              class="field-selection-dropdown__input w-full p-2"
                              part="field-select"
                              onChange={(e: any) => this.currentField = e.target.value}
                            >
                              <option value="" selected={!this.currentField}>Select Field...</option>
                              {this.availableFields.map(field => (
                                <option key={field} value={field} selected={this.currentField === field}>{field}</option>
                              ))}
                            </select>
                          </div>
                        </div>
                        <div class="field-selection-dropdown__indicators css-1wy0on6">
                          <span class="field-selection-dropdown__indicator-separator css-1u9des2-indicatorSeparator"></span>
                          <div class="field-selection-dropdown__indicator field-selection-dropdown__dropdown-indicator css-1xc3v61-indicatorContainer" aria-hidden="true">
                            <svg height="20" width="20" viewBox="0 0 20 20" aria-hidden="true" focusable="false" class="css-tj5bde-Svg">
                              <path d="M4.516 7.548c0.436-0.446 1.043-0.481 1.576 0l3.908 3.747 3.908-3.747c0.533-0.481 1.141-0.446 1.574 0 0.436 0.445 0.408 1.197 0 1.615-0.406 0.418-4.695 4.502-4.695 4.502-0.217 0.223-0.502 0.335-0.787 0.335s-0.57-0.112-0.789-0.335c0 0-4.287-4.084-4.695-4.502s-0.436-1.17 0-1.615z"></path>
                            </svg>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                )}
                
                {this.currentConditionType === 'field' && this.currentField && (
                  <div part="operator-selector">
                    <label class="block text-sm font-medium text-gray-700 mb-2" part="operator-label">
                      <slot name="operator-label">Select Operator:</slot>
                    </label>
                    <div class="css-b62m3t-container" part="operator-dropdown-container">
                      <span id="react-select-9-live-region" class="css-1f43avz-a11yText-A11yText"></span>
                      <span aria-live="polite" aria-atomic="false" aria-relevant="additions text" role="log" class="css-1f43avz-a11yText-A11yText"></span>
                      <div class="operator-selection-dropdown__control css-13cymwt-control">
                        <div class="operator-selection-dropdown__value-container operator-selection-dropdown__value-container--has-value css-hlgwow">
                          <div class="operator-selection-dropdown__single-value css-1dimb5e-singleValue">
                            {this.currentOperator || 'Select Operator...'}
                          </div>
                          <div class="operator-selection-dropdown__input-container css-19bb58m" data-value="">
                            <select 
                              class="operator-selection-dropdown__input w-full p-2"
                              part="operator-select"
                              onChange={(e: any) => this.currentOperator = e.target.value}
                            >
                              <option value="" selected={!this.currentOperator}>Select Operator...</option>
                              {this.localizedOperators.map(op => (
                                <option key={op.value} value={op.value} selected={this.currentOperator === op.value}>{op.label}</option>
                              ))}
                            </select>
                          </div>
                        </div>
                        <div class="operator-selection-dropdown__indicators css-1wy0on6">
                          <span class="operator-selection-dropdown__indicator-separator css-1u9des2-indicatorSeparator"></span>
                          <div class="operator-selection-dropdown__indicator operator-selection-dropdown__dropdown-indicator css-1xc3v61-indicatorContainer" aria-hidden="true">
                            <svg height="20" width="20" viewBox="0 0 20 20" aria-hidden="true" focusable="false" class="css-tj5bde-Svg">
                              <path d="M4.516 7.548c0.436-0.446 1.043-0.481 1.576 0l3.908 3.747 3.908-3.747c0.533-0.481 1.141-0.446 1.574 0 0.436 0.445 0.408 1.197 0 1.615-0.406 0.418-4.695 4.502-4.695 4.502-0.217 0.223-0.502 0.335-0.787 0.335s-0.57-0.112-0.789-0.335c0 0-4.287-4.084-4.695-4.502s-0.436-1.17 0-1.615z"></path>
                            </svg>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                )}
                
                {this.currentConditionType === 'field' && this.currentField && this.currentOperator && (
                  <div part="value-input-container">
                    <label class="block text-sm font-medium text-gray-700 mb-2" part="value-label">
                      <slot name="value-label">Enter Value:</slot>
                    </label>
                    <input 
                      data-testid="value-input" 
                      placeholder="e.g., &quot;text&quot;, 25, [1, 2, 3], true" 
                      class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent" 
                      part="value-input"
                      type="text" 
                      value={this.currentValue}
                      onInput={(e: any) => this.currentValue = e.target.value}
                    />
                    <p class="text-xs text-gray-500 mt-1" part="value-hint">
                      <slot name="value-hint">Tip: Use JSON format for arrays, objects, or boolean values</slot>
                    </p>
                  </div>
                )}
                
                {this.currentConditionType === 'where' && (
                  <div part="where-input-container">
                    <label class="block text-sm font-medium text-gray-700 mb-2" part="where-label">
                      <slot name="where-label">Enter Expression:</slot>
                    </label>
                    <input 
                      placeholder="e.g., this.age > 25" 
                      class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent" 
                      part="where-input"
                      type="text" 
                      value={this.currentWhereExpression}
                      onInput={(e: any) => this.currentWhereExpression = e.target.value}
                    />
                    <p class="text-xs text-gray-500 mt-1" part="where-hint">
                      <slot name="where-hint">Use "this" to refer to the document.</slot>
                    </p>
                  </div>
                )}
                
                {this.isAddFormReady() && (
                  <div class="pt-2 flex space-x-2" part="action-buttons">
                    <button 
                      class="flex-1 bg-blue-600 text-white py-2 rounded-lg" 
                      part="add-and-button"
                      onClick={() => this.applyCondition('and')}
                    >
                      <slot name="add-and-text">Add to AND</slot>
                    </button>
                    <button 
                      class="flex-1 bg-orange-600 text-white py-2 rounded-lg" 
                      part="add-or-button"
                      onClick={() => this.applyCondition('or')}
                    >
                      <slot name="add-or-text">Add to OR</slot>
                    </button>
                  </div>
                )}
              </div>
              
              <div class="bg-gray-900 text-gray-100 rounded-lg p-4" part="query-output">
                <p class="text-sm font-semibold mb-2" part="query-output-title">
                  <slot name="query-output-title">Generated MongoDB Query:</slot>
                </p>
                <pre class="text-sm overflow-x-auto" part="query-output-content">{this.value}</pre>
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  renderAddForm(isAddFormReady: boolean) {
    return !this.isAddingCondition ? (
      <div onClick={() => (this.isAddingCondition = true)}>
        <slot name="add-condition-button">
          <button class="w-full bg-green-100 border-2 border-dashed border-green-300 text-green-700 py-3 px-4 rounded-lg hover:bg-green-200 flex items-center justify-center space-x-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path></svg>
            <span>{this.localization?.addConditionButton || 'Add New Condition'}</span>
          </button>
        </slot>
      </div>
    ) : (
      <div class="bg-white border border-gray-200 rounded-lg p-4 space-y-4" part="add-form">
        <slot name="add-form-header">
          <div class="flex items-center justify-between"><h3 class="text-lg font-semibold">{this.localization?.addForm?.title || 'Add New Condition'}</h3><div onClick={() => this.resetAddForm()}>
              <slot name="close-add-form-button">
                <button class="text-gray-500 hover:text-gray-700"><svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg></button>
              </slot>
            </div></div>
        </slot>
        <slot name="add-form-body">
          <div><label class="block text-sm font-medium text-gray-700 mb-2">{this.localization?.addForm?.conditionType || 'Condition Type:'}</label><div class="flex space-x-4"><button onClick={() => this.currentConditionType = 'field'} class={`px-4 py-2 rounded text-sm font-medium ${this.currentConditionType === 'field' ? 'bg-blue-600 text-white' : 'bg-blue-100 text-blue-800'}`}>{this.localization?.addForm?.field || 'Field'}</button><button onClick={() => this.currentConditionType = 'where'} class={`px-4 py-2 rounded text-sm font-medium ${this.currentConditionType === 'where' ? 'bg-green-600 text-white' : 'bg-green-100 text-green-800'}`}>{this.localization?.addForm?.where || '$where'}</button></div></div>
          {this.currentConditionType === 'field' && <div class="space-y-3"><select onChange={(e: any) => this.currentField = e.target.value} class="w-full px-3 py-2 border border-gray-300 rounded-md"><option value="">{this.localization?.addForm?.selectField || 'Select Field...'}</option>{this.availableFields.map(f => <option value={f}>{f}</option>)}</select>{this.currentField && <select onChange={(e: any) => this.currentOperator = e.target.value} class="w-full px-3 py-2 border border-gray-300 rounded-md"><option value="">{this.localization?.addForm?.selectOperator || 'Select Operator...'}</option>{this.localizedOperators.map(o => <option value={o.value}>{o.label}</option>)}</select>}{this.currentOperator && <div><input type="text" value={this.currentValue} onInput={(e: any) => this.currentValue = e.target.value} placeholder={this.localization?.addForm?.valuePlaceholder || 'e.g., "text", 25'} class="w-full px-3 py-2 border border-gray-300 rounded-md" /><p class="text-xs text-gray-500 mt-1">{this.localization?.addForm?.valueTip || 'Tip: Use JSON format for arrays, booleans, etc.'}</p></div>}</div>}
          {this.currentConditionType === 'where' && <div><input type="text" value={this.currentWhereExpression} onInput={(e: any) => this.currentWhereExpression = e.target.value} placeholder={this.localization?.addForm?.wherePlaceholder || 'e.g., this.age > 25'} class="w-full px-3 py-2 border border-gray-300 rounded-md" /><p class="text-xs text-gray-500 mt-1">{this.localization?.addForm?.whereTip || 'Use "this" to refer to the document.'}</p></div>}
        </slot>
        <slot name="add-form-footer">
          {isAddFormReady && <div class="pt-2 flex space-x-2">
            <div onClick={() => this.applyCondition('and')}>
              <slot name="apply-and-button">
                <button class="flex-1 bg-blue-600 text-white py-2 rounded-lg">{this.localization?.addForm?.addToAnd || 'Add to AND'}</button>
              </slot>
            </div>
            <div onClick={() => this.applyCondition('or')}>
              <slot name="apply-or-button">
                <button class="flex-1 bg-orange-600 text-white py-2 rounded-lg">{this.localization?.addForm?.addToOr || 'Add to OR'}</button>
              </slot>
            </div>
          </div>}
        </slot>
      </div>
    );
  }

  renderEditForm() {
    if (!this.editingCondition) return null;

    const isEditFormReady = !!((this.editingCondition.type === 'field' && this.editingCondition.field && this.editingCondition.operator) || (this.editingCondition.type === 'where' && this.editingCondition.expression.trim()));

    const renderFieldConditionForm = (condition: FieldCondition) => {
        return (
            <div class="space-y-3">
                <select onChange={(e: any) => this.editingCondition = { ...condition, field: e.target.value }} class="w-full px-3 py-2 border border-gray-300 rounded-md">
                    {this.availableFields.map(f => <option value={f} selected={condition.field === f}>{f}</option>)}
                </select>
                <select onChange={(e: any) => this.editingCondition = { ...condition, operator: e.target.value }} class="w-full px-3 py-2 border border-gray-300 rounded-md">
                    {this.localizedOperators.map(o => <option value={o.value} selected={condition.operator === o.value}>{o.label}</option>)}
                </select>
                <div>
                    <input type="text" value={JSON.stringify(condition.value)} onInput={(e: any) => this.editingCondition = { ...condition, value: this.parseInputValue(e.target.value) }} class="w-full px-3 py-2 border border-gray-300 rounded-md" />
                    <p class="text-xs text-gray-500 mt-1">{this.localization?.addForm?.valueTip || 'Tip: Use JSON format for arrays, booleans, etc.'}</p>
                </div>
                <div class="flex space-x-4">
                    <label class="block text-sm font-medium text-gray-700">{this.localization?.editForm?.group || 'Group:'}</label>
                    <button onClick={() => this.editingCondition = { ...condition, group: 'and' }} class={`px-3 py-1 rounded text-sm ${condition.group === 'and' ? 'bg-blue-600 text-white' : 'bg-blue-100 text-blue-800'}`}>{this.localization?.editForm?.and || 'AND'}</button>
                    <button onClick={() => this.editingCondition = { ...condition, group: 'or' }} class={`px-3 py-1 rounded text-sm ${condition.group === 'or' ? 'bg-orange-600 text-white' : 'bg-orange-100 text-orange-800'}`}>{this.localization?.editForm?.or || 'OR'}</button>
                </div>
            </div>
        );
    }

    const renderWhereConditionForm = (condition: WhereCondition) => {
        return (
            <div>
                <input type="text" value={condition.expression} onInput={(e: any) => this.editingCondition = { ...condition, expression: e.target.value }} class="w-full px-3 py-2 border border-gray-300 rounded-md" />
            </div>
        );
    }

    return (
        <div class="bg-white border-2 border-blue-400 rounded-lg p-4 space-y-4" part="edit-form">
            <slot name="edit-form-header">
              <div class="flex items-center justify-between">
                  <h3 class="text-lg font-semibold text-blue-800">{this.localization?.editForm?.title || 'Edit Condition'}</h3>
                  <div onClick={() => this.cancelEditCondition()}>
                    <slot name="close-edit-form-button">
                      <button class="text-gray-500 hover:text-gray-700">
                          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                          </svg>
                      </button>
                    </slot>
                  </div>
              </div>
            </slot>
            <slot name="edit-form-body">
              {this.editingCondition.type === 'field' && renderFieldConditionForm(this.editingCondition)}
              {this.editingCondition.type === 'where' && renderWhereConditionForm(this.editingCondition)}
            </slot>
            <slot name="edit-form-footer">
              {isEditFormReady && <div class="pt-2 flex space-x-2">
                  <div onClick={() => this.saveEditCondition()}>
                    <slot name="save-changes-button">
                      <button class="flex-1 bg-green-600 text-white py-2 rounded-lg">{this.localization?.editForm?.save || 'Save Changes'}</button>
                    </slot>
                  </div>
                  <div onClick={() => this.cancelEditCondition()}>
                    <slot name="cancel-edit-button">
                      <button class="flex-1 bg-gray-500 text-white py-2 rounded-lg">{this.localization?.editForm?.cancel || 'Cancel'}</button>
                    </slot>
                  </div>
              </div>}
            </slot>
        </div>
    );
  }
}