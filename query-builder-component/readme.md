[![Built With Stencil](https://img.shields.io/badge/-Built%20With%20Stencil-16161d.svg?logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPCEtLSBHZW5lcmF0b3I6IEFkb2JlIElsbHVzdHJhdG9yIDE5LjIuMSwgU1ZHIEV4cG9ydCBQbHVnLUluIC4gU1ZHIFZlcnNpb246IDYuMDAgQnVpbGQgMCkgIC0tPgo8c3ZnIHZlcnNpb249IjEuMSIgaWQ9IkxheWVyXzEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiIHg9IjBweCIgeT0iMHB4IgoJIHZpZXdCb3g9IjAgMCA1MTIgNTEyIiBzdHlsZT0iZW5hYmxlLWJhY2tncm91bmQ6bmV3IDAgMCA1MTIgNTEyOyIgeG1sOnNwYWNlPSJwcmVzZXJ2ZSI%2BCjxzdHlsZSB0eXBlPSJ0ZXh0L2NzcyI%2BCgkuc3Qwe2ZpbGw6I0ZGRkZGRjt9Cjwvc3R5bGU%2BCjxwYXRoIGNsYXNzPSJzdDAiIGQ9Ik00MjQuNywzNzMuOWMwLDM3LjYtNTUuMSw2OC42LTkyLjcsNjguNkgxODAuNGMtMzcuOSwwLTkyLjctMzAuNy05Mi43LTY4LjZ2LTMuNmgzMzYuOVYzNzMuOXoiLz4KPHBhdGggY2xhc3M9InN0MCIgZD0iTTQyNC43LDI5Mi4xSDE4MC40Yy0zNy42LDAtOTIuNy0zMS05Mi43LTY4LjZ2LTMuNkgzMzJjMzcuNiwwLDkyLjcsMzEsOTIuNyw2OC42VjI5Mi4xeiIvPgo8cGF0aCBjbGFzcz0ic3QwIiBkPSJNNDI0LjcsMTQxLjdIODcuN3YtMy42YzAtMzcuNiw1NC44LTY4LjYsOTIuNy02OC42SDMzMmMzNy45LDAsOTIuNywzMC43LDkyLjcsNjguNlYxNDEuN3oiLz4KPC9zdmc%2BCg%3D%3D&colorA=16161d&style=flat-square)](https://stenciljs.com)

# MongoDB Query Builder Component

> A fully customizable, feature-rich MongoDB query builder web component built with Stencil.js. Create complex MongoDB queries through an intuitive UI with complete styling control via CSS parts and content customization through slots.

## 🚀 Features

- **Visual Query Building**: Build MongoDB queries through an intuitive UI
- **Field & Where Conditions**: Support for field-based conditions and `$where` expressions
- **Multiple Operators**: Full support for MongoDB operators ($eq, $ne, $gt, $gte, $lt, $lte, $in, $nin, $exists, $regex, $size, $type)
- **AND/OR Logic**: Combine conditions with AND/OR logic
- **Undo/Redo**: Full history support with undo/redo functionality
- **JSON Input Support**: Automatically extract fields from JSON data
- **Fully Customizable**: 60+ CSS parts and 15+ slots for complete customization
- **Shadow DOM**: Encapsulated styles with full customization support
- **TypeScript**: Built with TypeScript for type safety
- **Framework Agnostic**: Works with any framework or vanilla JavaScript

## 📦 Installation

### NPM
```bash
npm install query-builder-component
```

### CDN
```html
<script type="module" src="https://unpkg.com/query-builder-component/dist/query-builder-component/query-builder-component.esm.js"></script>
```

## 🎯 Quick Start

### Basic Usage

```html
<!DOCTYPE html>
<html>
<head>
  <script type="module" src="path/to/query-builder-component.esm.js"></script>
</head>
<body>
  <query-builder-component id="queryBuilder"></query-builder-component>
  
  <script>
    const queryBuilder = document.getElementById('queryBuilder');
    
    // Set input data to extract fields
    queryBuilder.jsonInput = [
      {
        name: "John Doe",
        age: 30,
        address: {
          city: "New York",
          country: "USA"
        },
        tags: ["customer", "vip"]
      }
    ];
    
    // Listen for query changes
    queryBuilder.addEventListener('queryChanged', (event) => {
      console.log('Query:', event.detail);
    });
  </script>
</body>
</html>
```

### React Example

```jsx
import React, { useRef, useEffect } from 'react';
import 'query-builder-component';

function App() {
  const queryBuilderRef = useRef(null);
  
  useEffect(() => {
    const element = queryBuilderRef.current;
    
    element.jsonInput = [{ /* your data */ }];
    
    element.addEventListener('queryChanged', (event) => {
      console.log('Query changed:', event.detail);
    });
  }, []);
  
  return (
    <query-builder-component ref={queryBuilderRef}></query-builder-component>
  );
}
```

## 🎨 Customization Guide

The Query Builder Component offers extensive customization through CSS parts and slots, allowing you to completely transform its appearance and behavior without modifying the source code.

### CSS Parts Reference

The component exposes 60+ CSS parts for styling. Here's a complete reference:

#### Container Parts
```css
/* Main containers */
query-builder-component::part(root-container) { /* Root wrapper */ }
query-builder-component::part(main-panel) { /* Main panel */ }
query-builder-component::part(content-wrapper) { /* Content area */ }
query-builder-component::part(query-builder-container) { /* Query builder container */ }
```

#### Header Parts
```css
/* Header section */
query-builder-component::part(header-section) { /* Header container */ }
query-builder-component::part(header-title) { /* Header title */ }
query-builder-component::part(header-description) { /* Header description */ }
```

#### Toolbar Parts
```css
/* Toolbar */
query-builder-component::part(toolbar) { /* Toolbar container */ }
query-builder-component::part(toolbar-left) { /* Left side of toolbar */ }
query-builder-component::part(toolbar-title) { /* Toolbar title */ }
query-builder-component::part(history-controls) { /* Undo/redo container */ }
query-builder-component::part(undo-button) { /* Undo button */ }
query-builder-component::part(redo-button) { /* Redo button */ }
query-builder-component::part(undo-icon-svg) { /* Undo icon SVG */ }
query-builder-component::part(redo-icon-svg) { /* Redo icon SVG */ }
```

#### Conditions Display Parts
```css
/* Conditions list */
query-builder-component::part(conditions-container) { /* Conditions container */ }
query-builder-component::part(conditions-header) { /* Conditions header */ }
query-builder-component::part(conditions-title) { /* Conditions title */ }
query-builder-component::part(clear-all-button) { /* Clear all button */ }
query-builder-component::part(conditions-list) { /* Conditions list */ }

/* Individual condition */
query-builder-component::part(condition-row) { /* Condition row */ }
query-builder-component::part(condition-content) { /* Condition content */ }
query-builder-component::part(condition-group-badge) { /* AND/OR badge */ }
query-builder-component::part(condition-where-badge) { /* WHERE badge */ }
query-builder-component::part(condition-field) { /* Field name */ }
query-builder-component::part(condition-operator) { /* Operator */ }
query-builder-component::part(condition-value) { /* Value */ }
query-builder-component::part(condition-actions) { /* Actions container */ }
query-builder-component::part(edit-button) { /* Edit button */ }
query-builder-component::part(delete-button) { /* Delete button */ }
query-builder-component::part(edit-icon-svg) { /* Edit icon SVG */ }
query-builder-component::part(delete-icon-svg) { /* Delete icon SVG */ }
```

#### Add Condition Form Parts
```css
/* Add condition form */
query-builder-component::part(add-condition-container) { /* Form container */ }
query-builder-component::part(add-condition-header) { /* Form header */ }
query-builder-component::part(add-condition-title) { /* Form title */ }
query-builder-component::part(close-add-button) { /* Close button */ }
query-builder-component::part(close-icon-svg) { /* Close icon SVG */ }

/* Condition type selector */
query-builder-component::part(condition-type-selector) { /* Type selector container */ }
query-builder-component::part(condition-type-label) { /* Type label */ }
query-builder-component::part(condition-type-buttons) { /* Type buttons container */ }
query-builder-component::part(field-condition-button) { /* Field condition button */ }
query-builder-component::part(where-expression-button) { /* Where expression button */ }

/* Input fields */
query-builder-component::part(field-selector) { /* Field selector container */ }
query-builder-component::part(field-label) { /* Field label */ }
query-builder-component::part(field-dropdown-container) { /* Field dropdown container */ }
query-builder-component::part(field-select) { /* Field select element */ }

query-builder-component::part(operator-selector) { /* Operator selector container */ }
query-builder-component::part(operator-label) { /* Operator label */ }
query-builder-component::part(operator-dropdown-container) { /* Operator dropdown container */ }
query-builder-component::part(operator-select) { /* Operator select element */ }

query-builder-component::part(value-input-container) { /* Value input container */ }
query-builder-component::part(value-label) { /* Value label */ }
query-builder-component::part(value-input) { /* Value input field */ }
query-builder-component::part(value-hint) { /* Value hint text */ }

query-builder-component::part(where-input-container) { /* Where input container */ }
query-builder-component::part(where-label) { /* Where label */ }
query-builder-component::part(where-input) { /* Where input field */ }
query-builder-component::part(where-hint) { /* Where hint text */ }

/* Action buttons */
query-builder-component::part(action-buttons) { /* Action buttons container */ }
query-builder-component::part(add-and-button) { /* Add to AND button */ }
query-builder-component::part(add-or-button) { /* Add to OR button */ }
```

#### Query Output Parts
```css
/* Query output */
query-builder-component::part(query-output) { /* Output container */ }
query-builder-component::part(query-output-title) { /* Output title */ }
query-builder-component::part(query-output-content) { /* Output content */ }
```

### Slots Reference

Slots allow you to replace content and icons with your own HTML:

#### Content Slots
```html
<query-builder-component>
  <!-- Header -->
  <div slot="header">
    <h2>Custom Header</h2>
    <p>Custom description</p>
  </div>
  
  <!-- Text replacements -->
  <span slot="add-condition-title">➕ New Condition</span>
  <span slot="clear-all-text">Clear All</span>
  <span slot="condition-type-label">Choose Type:</span>
  <span slot="field-condition-text">Field Based</span>
  <span slot="where-expression-text">Custom Expression</span>
  <span slot="field-label">Select Field:</span>
  <span slot="operator-label">Select Operator:</span>
  <span slot="value-label">Enter Value:</span>
  <span slot="value-hint">Tip: Use JSON format</span>
  <span slot="where-label">Enter Expression:</span>
  <span slot="where-hint">Use "this" to refer to the document</span>
  <span slot="add-and-text">ADD WITH AND</span>
  <span slot="add-or-text">ADD WITH OR</span>
  <span slot="query-output-title">Generated Query:</span>
</query-builder-component>
```

#### Icon Slots
```html
<query-builder-component>
  <!-- Custom icons -->
  <svg slot="undo-icon" class="custom-icon">
    <!-- Your custom undo icon SVG -->
  </svg>
  
  <svg slot="redo-icon" class="custom-icon">
    <!-- Your custom redo icon SVG -->
  </svg>
  
  <svg slot="edit-icon" class="custom-icon">
    <!-- Your custom edit icon SVG -->
  </svg>
  
  <svg slot="delete-icon" class="custom-icon">
    <!-- Your custom delete icon SVG -->
  </svg>
  
  <svg slot="close-icon" class="custom-icon">
    <!-- Your custom close icon SVG -->
  </svg>
</query-builder-component>
```

## 🎭 Theme Examples

### Dark Theme
```css
/* Dark theme styling */
query-builder-component::part(root-container) {
  background: #1f2937;
  color: #f3f4f6;
}

query-builder-component::part(main-panel) {
  background: #111827;
  border: 1px solid #374151;
}

query-builder-component::part(header-section) {
  background: #1f2937;
  border-bottom: 1px solid #374151;
}

query-builder-component::part(header-title) {
  color: #f9fafb;
}

query-builder-component::part(condition-row) {
  background: #111827;
  border-color: #374151;
}

query-builder-component::part(condition-field) {
  color: #60a5fa;
}

query-builder-component::part(condition-operator) {
  color: #34d399;
}

query-builder-component::part(condition-value) {
  color: #c084fc;
}

query-builder-component::part(query-output) {
  background: #030712;
  color: #f3f4f6;
}
```

### Material Design Theme
```css
/* Material Design inspired theme */
query-builder-component::part(main-panel) {
  box-shadow: 0 3px 6px rgba(0,0,0,0.16), 0 3px 6px rgba(0,0,0,0.23);
  border: none;
}

query-builder-component::part(header-section) {
  background: #6200ea;
  padding: 1.5rem;
  color: white;
}

query-builder-component::part(undo-button),
query-builder-component::part(redo-button) {
  border-radius: 50%;
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.3s;
}

query-builder-component::part(undo-button):hover {
  background: rgba(98, 0, 234, 0.08);
}

query-builder-component::part(add-and-button),
query-builder-component::part(add-or-button) {
  text-transform: uppercase;
  letter-spacing: 1px;
  font-weight: 500;
  box-shadow: 0 2px 4px rgba(0,0,0,0.2);
}
```

### Gradient Theme
```css
/* Beautiful gradient theme */
query-builder-component::part(header-section) {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 1.5rem;
}

query-builder-component::part(toolbar) {
  background: linear-gradient(90deg, #f093fb 0%, #f5576c 100%);
  border-radius: 8px;
}

query-builder-component::part(add-and-button) {
  background: linear-gradient(45deg, #2196F3 30%, #21CBF3 90%);
  box-shadow: 0 3px 5px 2px rgba(33, 203, 243, .3);
}

query-builder-component::part(add-or-button) {
  background: linear-gradient(45deg, #FE6B8B 30%, #FF8E53 90%);
  box-shadow: 0 3px 5px 2px rgba(255, 105, 135, .3);
}
```

## 📋 API Reference

### Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `value` | `string` | The current MongoDB query as JSON string | `'{}'` |
| `availableFields` | `string[]` | Array of field names available for querying | `[]` |
| `jsonInput` | `object \| any[]` | JSON data to extract fields from | `null` |
| `renderCondition` | `(condition: QueryCondition) => string` | Custom render function for conditions | `undefined` |
| `localization` | `object` | Localization object for text and operators | `undefined` |

### Events

| Event | Detail | Description |
|-------|--------|-------------|
| `queryChanged` | `string` | Emitted when the query changes |
| `historyUpdated` | `{ canUndo: boolean; canRedo: boolean }` | Emitted when history state changes |

### Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `undo()` | `Promise<void>` | Undo the last query change |
| `redo()` | `Promise<void>` | Redo the previously undone change |

### Localization

```javascript
const localization = {
  operators: {
    '$eq': 'equals',
    '$ne': 'not equals',
    '$gt': 'greater than',
    '$gte': 'greater or equal',
    '$lt': 'less than',
    '$lte': 'less or equal',
    '$in': 'in array',
    '$nin': 'not in array',
    '$exists': 'exists',
    '$regex': 'matches pattern',
    '$size': 'array size',
    '$type': 'type is'
  },
  addForm: {
    title: 'Add New Condition',
    // ... other form labels
  }
};

queryBuilder.localization = localization;
```

## 🚧 Advanced Usage

### Custom Condition Rendering

You can provide a custom render function to change how conditions are displayed:

```javascript
queryBuilder.renderCondition = (condition) => {
  if (condition.type === 'field') {
    return `
      <div class="custom-condition">
        <span class="field">${condition.field}</span>
        <span class="operator">${condition.operator}</span>
        <span class="value">${condition.value}</span>
      </div>
    `;
  } else {
    return `
      <div class="custom-where">
        <code>${condition.expression}</code>
      </div>
    `;
  }
};
```

### Programmatic Query Setting

```javascript
// Set a query programmatically
queryBuilder.value = JSON.stringify({
  "$and": [
    { "age": { "$gte": 25 } },
    { "department": { "$eq": "Engineering" } }
  ]
}, null, 2);
```

### Framework Integration Examples

#### Vue 3
```vue
<template>
  <query-builder-component 
    ref="queryBuilder"
    @queryChanged="handleQueryChange"
  ></query-builder-component>
</template>

<script>
import { ref, onMounted } from 'vue';
import 'query-builder-component';

export default {
  setup() {
    const queryBuilder = ref(null);
    
    onMounted(() => {
      queryBuilder.value.jsonInput = [/* your data */];
    });
    
    const handleQueryChange = (event) => {
      console.log('Query:', event.detail);
    };
    
    return {
      queryBuilder,
      handleQueryChange
    };
  }
};
</script>
```

#### Angular
```typescript
import { Component, ViewChild, ElementRef, AfterViewInit } from '@angular/core';
import 'query-builder-component';

@Component({
  selector: 'app-root',
  template: `
    <query-builder-component #queryBuilder></query-builder-component>
  `
})
export class AppComponent implements AfterViewInit {
  @ViewChild('queryBuilder') queryBuilder: ElementRef;
  
  ngAfterViewInit() {
    const element = this.queryBuilder.nativeElement;
    
    element.jsonInput = [/* your data */];
    
    element.addEventListener('queryChanged', (event: CustomEvent) => {
      console.log('Query:', event.detail);
    });
  }
}
```

## 🐛 Troubleshooting

### Common Issues

1. **Component not rendering**: Ensure the script is loaded as a module:
   ```html
   <script type="module" src="..."></script>
   ```

2. **Styles not applying**: Remember that the component uses Shadow DOM. Use `::part()` selectors:
   ```css
   /* Wrong */
   query-builder-component .header { }
   
   /* Correct */
   query-builder-component::part(header-section) { }
   ```

3. **Slots not working**: Ensure slot content is direct children of the component:
   ```html
   <!-- Wrong -->
   <query-builder-component>
     <div>
       <span slot="header">Header</span>
     </div>
   </query-builder-component>
   
   <!-- Correct -->
   <query-builder-component>
     <span slot="header">Header</span>
   </query-builder-component>
   ```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Stencil.js](https://stenciljs.com/)
- Inspired by MongoDB's query language
- Icons from [Lucide](https://lucide.dev/)

## 📧 Support

For support, please open an issue in the GitHub repository.
