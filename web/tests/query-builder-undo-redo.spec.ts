import { test, expect, Page } from '@playwright/test';

test.describe('QueryBuilder Undo/Redo Functionality', () => {
  let page: Page;
  let consoleMessages: string[] = [];
  let networkRequests: string[] = [];

  test.beforeEach(async ({ page: testPage }) => {
    page = testPage;
    consoleMessages = [];
    networkRequests = [];

    // Capture console messages to detect potential issues
    page.on('console', msg => {
      consoleMessages.push(`${msg.type()}: ${msg.text()}`);
    });

    // Capture network requests to detect excessive re-renders
    page.on('request', request => {
      networkRequests.push(request.url());
    });

    // Navigate to the page
    await page.goto('/');
    
    // Wait for the page to be fully loaded
    await page.waitForLoadState('networkidle');
    
    // Wait for the QueryBuilder to be loaded
    await page.waitForSelector('[data-testid="query-builder"]', { timeout: 10000 });
    
    // Wait for the available fields to be extracted from the JSON
    await page.waitForTimeout(1000);
  });

  test('should not create circular updates when adding conditions and using undo/redo', async () => {
    console.log('Starting circular update test...');

    // Step 1: Add a field condition
    await page.click('[data-testid="add-new-condition-button"]');
    await page.waitForSelector('[data-testid="field-condition-button"]', { state: 'visible' });
    await page.click('[data-testid="field-condition-button"]');
    
    // Wait for the form to appear
    await page.waitForSelector('label:has-text("Select Field:")', { state: 'visible' });
    
    // Select field - find the field selection dropdown
    await page.click('.field-selection-dropdown__control');
    await page.waitForSelector('.field-selection-dropdown__menu', { state: 'visible' });
    await page.click('.field-selection-dropdown__option:has-text("name")');
    
    // Wait for operator dropdown to appear
    await page.waitForSelector('label:has-text("Select Operator:")', { state: 'visible' });
    
    // Select operator - find the operator selection dropdown
    await page.click('.operator-selection-dropdown__control');
    await page.waitForSelector('.operator-selection-dropdown__menu', { state: 'visible' });
    await page.click('.operator-selection-dropdown__option:has-text("$eq - field equals value")');
    
    // Wait for value input to appear
    await page.waitForSelector('label:has-text("Enter Value:")', { state: 'visible' });
    
    // Enter value
    await page.fill('[data-testid="value-input"]', '"John"');
    
    // Add to AND group
    await page.waitForSelector('[data-testid="add-to-and-group-button"]', { state: 'visible' });
    await page.click('[data-testid="add-to-and-group-button"]');
    
    // Wait for condition to be added
    await page.waitForTimeout(300);
    
    console.log(`Console messages after adding condition: ${consoleMessages.length}`);
    
    // Step 2: Add another condition
    await page.click('[data-testid="add-new-condition-button"]');
    await page.waitForSelector('[data-testid="field-condition-button"]', { state: 'visible' });
    await page.click('[data-testid="field-condition-button"]');
    
    // Wait for the form to appear
    await page.waitForSelector('label:has-text("Select Field:")', { state: 'visible' });
    
    // Select field (age)
    await page.click('.field-selection-dropdown__control');
    await page.waitForSelector('.field-selection-dropdown__menu', { state: 'visible' });
    await page.click('.field-selection-dropdown__option:has-text("age")');
    
    // Wait for operator dropdown to appear
    await page.waitForSelector('label:has-text("Select Operator:")', { state: 'visible' });
    
    // Select operator ($gt)
    await page.click('.operator-selection-dropdown__control');
    await page.waitForSelector('.operator-selection-dropdown__menu', { state: 'visible' });
    await page.click('.operator-selection-dropdown__option:has-text("$gt - field > value")');
    
    // Wait for value input to appear
    await page.waitForSelector('label:has-text("Enter Value:")', { state: 'visible' });
    
    // Enter value
    await page.fill('[data-testid="value-input"]', '25');
    
    // Add to OR group
    await page.waitForSelector('[data-testid="add-to-or-group-button"]', { state: 'visible' });
    await page.click('[data-testid="add-to-or-group-button"]');
    
    // Wait for condition to be added
    await page.waitForTimeout(300);
    
    console.log(`Console messages after adding second condition: ${consoleMessages.length}`);
    
    // Step 3: Test undo functionality
    const initialConsoleCount = consoleMessages.length;
    
    // Click undo button
    await page.click('[data-testid="undo-button"]');
    
    // Wait a moment for any potential circular updates
    await page.waitForTimeout(500);
    
    const afterUndoConsoleCount = consoleMessages.length;
    
    // Click undo again
    await page.click('[data-testid="undo-button"]');
    
    // Wait a moment for any potential circular updates
    await page.waitForTimeout(500);
    
    const afterSecondUndoConsoleCount = consoleMessages.length;
    
    console.log(`Console messages - Initial: ${initialConsoleCount}, After first undo: ${afterUndoConsoleCount}, After second undo: ${afterSecondUndoConsoleCount}`);
    
    // Step 4: Test redo functionality
    const beforeRedoConsoleCount = consoleMessages.length;
    
    // Click redo button
    await page.click('[data-testid="redo-button"]');
    
    // Wait a moment for any potential circular updates
    await page.waitForTimeout(500);
    
    const afterRedoConsoleCount = consoleMessages.length;
    
    // Click redo again
    await page.click('[data-testid="redo-button"]');
    
    // Wait a moment for any potential circular updates
    await page.waitForTimeout(500);
    
    const afterSecondRedoConsoleCount = consoleMessages.length;
    
    console.log(`Console messages - Before redo: ${beforeRedoConsoleCount}, After first redo: ${afterRedoConsoleCount}, After second redo: ${afterSecondRedoConsoleCount}`);
    
    // Step 5: Analyze results
    console.log('All console messages:', consoleMessages);
    
    // Check for error messages indicating circular updates
    const errorMessages = consoleMessages.filter(msg => 
      msg.includes('error') || 
      msg.includes('Maximum update depth exceeded') ||
      msg.includes('Warning: setState')
    );
    
    if (errorMessages.length > 0) {
      console.log('Error messages found:', errorMessages);
    }
    
    // Check for excessive console output which might indicate re-render issues
    const suspiciousMessageCount = afterSecondRedoConsoleCount - initialConsoleCount;
    
    // The test should fail if we detect signs of circular updates
    expect(errorMessages.length).toBe(0);
    expect(suspiciousMessageCount).toBeLessThan(20); // Arbitrary threshold for "too many" messages
  });

  test('should handle keyboard shortcuts without circular updates', async () => {
    console.log('Starting keyboard shortcut test...');

    // Add conditions first
    await page.click('button:has-text("Add New Condition")');
    await page.click('button:has-text("Field Condition")');
    await page.click('[class*="react-select"]');
    await page.click('text=name');
    await page.click('[class*="react-select"]:not(:has([aria-selected="true"]))');
    await page.click('text=$eq - field equals value');
    await page.fill('input[placeholder*="text"]', '"Test"');
    await page.click('button:has-text("Add to AND Group")');
    
    const initialConsoleCount = consoleMessages.length;
    
    // Test Ctrl+Z (undo)
    await page.keyboard.press('Control+z');
    await page.waitForTimeout(200);
    
    const afterCtrlZConsoleCount = consoleMessages.length;
    
    // Test Ctrl+Y (redo)
    await page.keyboard.press('Control+y');
    await page.waitForTimeout(200);
    
    const afterCtrlYConsoleCount = consoleMessages.length;
    
    console.log(`Keyboard test - Initial: ${initialConsoleCount}, After Ctrl+Z: ${afterCtrlZConsoleCount}, After Ctrl+Y: ${afterCtrlYConsoleCount}`);
    
    // Check for error messages
    const errorMessages = consoleMessages.filter(msg => 
      msg.includes('error') || 
      msg.includes('Maximum update depth exceeded')
    );
    
    expect(errorMessages.length).toBe(0);
  });

  test('should handle rapid undo/redo operations without breaking', async () => {
    console.log('Starting rapid operations test...');

    // Add multiple conditions
    for (let i = 0; i < 3; i++) {
      await page.click('button:has-text("Add New Condition")');
      await page.click('button:has-text("Field Condition")');
      await page.click('[class*="react-select"]');
      await page.click('text=name');
      await page.click('[class*="react-select"]:not(:has([aria-selected="true"]))');
      await page.click('text=$eq - field equals value');
      await page.fill('input[placeholder*="text"]', `"Test${i}"`);
      await page.click('button:has-text("Add to AND Group")');
    }
    
    const initialConsoleCount = consoleMessages.length;
    
    // Rapid undo operations
    for (let i = 0; i < 3; i++) {
      await page.click('button[title="Undo (Ctrl+Z)"]');
      await page.waitForTimeout(50); // Very short wait to simulate rapid clicking
    }
    
    // Rapid redo operations
    for (let i = 0; i < 3; i++) {
      await page.click('button[title="Redo (Ctrl+Y)"]');
      await page.waitForTimeout(50);
    }
    
    const finalConsoleCount = consoleMessages.length;
    
    console.log(`Rapid operations test - Initial: ${initialConsoleCount}, Final: ${finalConsoleCount}`);
    console.log('Console messages:', consoleMessages);
    
    // Check for error messages
    const errorMessages = consoleMessages.filter(msg => 
      msg.includes('error') || 
      msg.includes('Maximum update depth exceeded') ||
      msg.includes('Warning: setState')
    );
    
    expect(errorMessages.length).toBe(0);
  });

  test('should maintain correct state during edit operations with undo/redo', async () => {
    console.log('Starting edit operations test...');

    // Add a condition
    await page.click('button:has-text("Add New Condition")');
    await page.click('button:has-text("Field Condition")');
    await page.click('[class*="react-select"]');
    await page.click('text=name');
    await page.click('[class*="react-select"]:not(:has([aria-selected="true"]))');
    await page.click('text=$eq - field equals value');
    await page.fill('input[placeholder*="text"]', '"Original"');
    await page.click('button:has-text("Add to AND Group")');
    
    // Edit the condition
    await page.click('button[title="Edit condition"]');
    await page.fill('input[value="Original"]', '"Modified"');
    await page.click('button:has-text("Save Changes")');
    
    const beforeUndoConsoleCount = consoleMessages.length;
    
    // Undo the edit
    await page.click('button[title="Undo (Ctrl+Z)"]');
    await page.waitForTimeout(200);
    
    const afterUndoConsoleCount = consoleMessages.length;
    
    // Redo the edit
    await page.click('button[title="Redo (Ctrl+Y)"]');
    await page.waitForTimeout(200);
    
    const afterRedoConsoleCount = consoleMessages.length;
    
    console.log(`Edit test - Before undo: ${beforeUndoConsoleCount}, After undo: ${afterUndoConsoleCount}, After redo: ${afterRedoConsoleCount}`);
    
    // Check for error messages
    const errorMessages = consoleMessages.filter(msg => 
      msg.includes('error') || 
      msg.includes('Maximum update depth exceeded')
    );
    
    expect(errorMessages.length).toBe(0);
  });
});
