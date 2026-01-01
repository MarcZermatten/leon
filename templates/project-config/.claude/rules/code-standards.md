---
paths: src/**/*.svelte, src/**/*.ts, src-tauri/**/*.rs
---

# Standards de Code Léon

## Svelte 5

### Props
```svelte
<!-- CORRECT -->
let { prop1, prop2 = 'default' } = $props<{
    prop1: string;
    prop2?: string;
}>();

<!-- INCORRECT (Svelte 4) -->
export let prop1: string;
```

### State
```svelte
<!-- CORRECT -->
let count = $state(0);
let doubled = $derived(count * 2);

<!-- INCORRECT -->
import { writable } from 'svelte/store';
const count = writable(0);
```

### Effects
```svelte
<!-- CORRECT -->
$effect(() => {
    console.log('count changed:', count);
    return () => cleanup();
});

<!-- INCORRECT -->
onMount(() => { ... });  // Seulement si vraiment nécessaire
```

## TypeScript

### Imports Tauri
```typescript
// API Core
import { invoke } from '@tauri-apps/api/core';

// Events
import { listen, emit } from '@tauri-apps/api/event';

// Plugins
import { readTextFile } from '@tauri-apps/plugin-fs';
import { open } from '@tauri-apps/plugin-dialog';
```

### Types Stricts
```typescript
// CORRECT
function process(data: ProcessInput): ProcessOutput { ... }

// INCORRECT
function process(data: any): any { ... }
```

## Rust/Tauri

### Commands
```rust
#[command]
pub async fn my_command(
    app: AppHandle,
    param: String,
) -> Result<ResponseType, String> {
    // ...
}
```

### State
```rust
// Définir
pub struct MyState {
    data: HashMap<String, Value>,
}

// Utiliser
let state = app.state::<Arc<Mutex<MyState>>>();
let mut data = state.lock().map_err(|e| e.to_string())?;
```

### Events
```rust
// Émettre
app.emit("event_name", serde_json::json!({
    "field": value
}))?;
```

## Performance

### Debounce Events Fréquents
```typescript
let debounceTimer: ReturnType<typeof setTimeout>;
function handleFrequentEvent(data: any) {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
        actualHandler(data);
    }, 100);
}
```

### Lazy Loading
```svelte
{#await import('./HeavyComponent.svelte') then module}
    <module.default />
{/await}
```

## Git

### Commits
```
type: description courte (max 50 chars)

Types: feat, fix, refactor, docs, style, chore, test
```

### Branches
```
feature/nom-feature
fix/description-bug
refactor/zone-refactoree
```
