# my-component



<!-- Auto Generated Below -->


## Properties

| Property          | Attribute          | Description | Type                                    | Default     |
| ----------------- | ------------------ | ----------- | --------------------------------------- | ----------- |
| `availableFields` | `available-fields` |             | `string[]`                              | `[]`        |
| `jsonInput`       | `json-input`       |             | `any[] \| object`                       | `null`      |
| `localization`    | `localization`     |             | `any`                                   | `undefined` |
| `renderCondition` | `render-condition` |             | `(condition: QueryCondition) => string` | `undefined` |
| `value`           | `value`            |             | `string`                                | `'{}'`      |


## Events

| Event            | Description | Type                                                   |
| ---------------- | ----------- | ------------------------------------------------------ |
| `historyUpdated` |             | `CustomEvent<{ canUndo: boolean; canRedo: boolean; }>` |
| `queryChanged`   |             | `CustomEvent<string>`                                  |


## Methods

### `redo() => Promise<void>`



#### Returns

Type: `Promise<void>`



### `undo() => Promise<void>`



#### Returns

Type: `Promise<void>`




## Shadow Parts

| Part          | Description |
| ------------- | ----------- |
| `"add-form"`  |             |
| `"edit-form"` |             |


----------------------------------------------

*Built with [StencilJS](https://stenciljs.com/)*
