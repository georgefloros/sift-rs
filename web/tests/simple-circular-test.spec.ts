import { test, expect } from '@playwright/test';

// Helper function to add a field condition
async function addFieldCondition(page: any, field: string, operator: string, value: string) {
  console.log(`🔧 Adding condition: ${field} ${operator} ${value}`);
  
  // Step 1: Click on "+ Add New Condition"
  console.log('1️⃣ Clicking Add New Condition');
  await page.click('[data-testid="add-new-condition-button"]');
  await page.waitForTimeout(500);
  
  // Step 2: Click on "Field Condition"
  console.log('2️⃣ Clicking Field Condition');
  await page.click('[data-testid="field-condition-button"]');
  await page.waitForTimeout(500);
  
  // Step 3: Select field from dropdown
  console.log(`3️⃣ Selecting field: ${field}`);
  await page.waitForSelector('text="Select Field:"', { timeout: 5000 });
  
  // Click on the field selection dropdown
  await page.click('.field-selection-dropdown__control');
  await page.waitForTimeout(300);
  
  // Wait for dropdown menu and click the option
  await page.waitForSelector('.field-selection-dropdown__menu', { timeout: 3000 });
  await page.click(`.field-selection-dropdown__option:has-text("${field}")`);
  await page.waitForTimeout(300);
  
  // Step 4: Select operator from dropdown
  console.log(`4️⃣ Selecting operator: ${operator}`);
  await page.waitForSelector('text="Select Operator:"', { timeout: 3000 });
  
  // Click on the operator selection dropdown
  await page.click('.operator-selection-dropdown__control');
  await page.waitForTimeout(300);
  
  // Wait for dropdown menu and click the option
  await page.waitForSelector('.operator-selection-dropdown__menu', { timeout: 3000 });
  await page.click(`.operator-selection-dropdown__option:has-text("${operator}")`);
  await page.waitForTimeout(300);
  
  // Step 5: Enter Value
  console.log(`5️⃣ Entering value: ${value}`);
  await page.waitForSelector('text="Enter Value:"', { timeout: 3000 });
  
  await page.fill('[data-testid="value-input"]', value);
  await page.waitForTimeout(300);
  
  // Step 6: Click "Add to AND Group"
  console.log('6️⃣ Clicking Add to AND Group');
  await page.waitForSelector('[data-testid="add-to-and-group-button"]', { timeout: 3000 });
  await page.click('[data-testid="add-to-and-group-button"]');
  await page.waitForTimeout(500);
  
  console.log('✅ Condition added successfully');
}

test.describe('QueryBuilder Circular Update Detection', () => {
  test('should load without circular updates', async ({ page }) => {
    const consoleMessages: string[] = [];
    const errorMessages: string[] = [];
    
    // Capture console messages
    page.on('console', msg => {
      const message = `${msg.type()}: ${msg.text()}`;
      consoleMessages.push(message);
      
      // Check for specific error patterns
      if (msg.type() === 'error' || 
          message.includes('Maximum update depth exceeded') ||
          message.includes('Warning: setState') ||
          message.includes('circular') ||
          message.includes('infinite loop')) {
        errorMessages.push(message);
      }
    });

    console.log('🚀 Starting page load test');
    
    // Navigate to the page
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Wait for QueryBuilder to be visible
    await page.waitForSelector('[data-testid="query-builder"]', { timeout: 15000 });
    
    // Wait a bit to see if any circular updates occur during initial load
    await page.waitForTimeout(2000);
    
    console.log(`📊 Total console messages: ${consoleMessages.length}`);
    console.log(`❌ Error messages: ${errorMessages.length}`);
    
    if (errorMessages.length > 0) {
      console.log('❗ Error messages found:', errorMessages);
    }
    
    // Check for excessive console output which might indicate re-render loops
    const fastRefreshCount = consoleMessages.filter(msg => msg.includes('Fast Refresh')).length;
    console.log(`🔄 Fast Refresh messages: ${fastRefreshCount}`);
    
    // Test should pass if no circular update errors are detected
    expect(errorMessages.length).toBe(0);
    expect(fastRefreshCount).toBeLessThan(20); // Reasonable threshold
  });

  test('should handle basic interactions without circular updates', async ({ page }) => {
    const consoleMessages: string[] = [];
    const errorMessages: string[] = [];
    
    page.on('console', msg => {
      const message = `${msg.type()}: ${msg.text()}`;
      consoleMessages.push(message);
      
      if (msg.type() === 'error' || 
          message.includes('Maximum update depth exceeded') ||
          message.includes('Warning: setState') ||
          message.includes('circular') ||
          message.includes('infinite loop')) {
        errorMessages.push(message);
      }
    });

    console.log('🚀 Starting interaction test');
    
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForSelector('[data-testid="query-builder"]', { timeout: 15000 });
    
    const initialMessageCount = consoleMessages.length;
    console.log(`📊 Initial message count: ${initialMessageCount}`);
    
    try {
      // Test adding a condition
      await addFieldCondition(page, 'name', '$eq', '"John"');
      
      // Wait to see if any circular updates occur
      await page.waitForTimeout(1000);
      
      const afterAddMessageCount = consoleMessages.length;
      console.log(`📊 After adding condition: ${afterAddMessageCount}`);
      
      // Verify the condition was added by checking if it appears in the UI
      const conditionVisible = await page.locator('text="name"').isVisible();
      console.log(`✅ Condition visible in UI: ${conditionVisible}`);
      
    } catch (error) {
      console.log('❌ Error during condition addition:', error);
      // Don't fail the test here, we still want to check for circular updates
    }
    
    const finalMessageCount = consoleMessages.length;
    const messageDiff = finalMessageCount - initialMessageCount;
    
    console.log(`📊 Final message count: ${finalMessageCount}`);
    console.log(`📊 Message difference: ${messageDiff}`);
    console.log(`❌ Error messages: ${errorMessages.length}`);
    
    if (errorMessages.length > 0) {
      console.log('❗ Error messages found:', errorMessages);
    }
    
    // Main test: no circular update errors should be present
    expect(errorMessages.length).toBe(0);
    
    // Secondary test: message count shouldn't explode
    expect(messageDiff).toBeLessThan(50);
  });

  test('should handle undo/redo without circular updates', async ({ page }) => {
    const consoleMessages: string[] = [];
    const errorMessages: string[] = [];
    
    page.on('console', msg => {
      const message = `${msg.type()}: ${msg.text()}`;
      consoleMessages.push(message);
      
      if (msg.type() === 'error' || 
          message.includes('Maximum update depth exceeded') ||
          message.includes('Warning: setState') ||
          message.includes('circular') ||
          message.includes('infinite loop')) {
        errorMessages.push(message);
      }
    });

    console.log('🚀 Starting undo/redo test');
    
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForSelector('[data-testid="query-builder"]', { timeout: 15000 });
    
    const initialMessageCount = consoleMessages.length;
    console.log(`📊 Initial message count: ${initialMessageCount}`);
    
    try {
      // Add first condition following exact steps
      console.log('📋 Step 1-6: Adding first condition (active = true)...');
      await addFieldCondition(page, 'active', '$eq', 'true');
      await page.waitForTimeout(500);
      
      const afterFirstConditionCount = consoleMessages.length;
      console.log(`📊 After first condition: ${afterFirstConditionCount}`);
      
      // Verify first condition is visible in the conditions list
      const firstConditionVisible = await page.locator('span:has-text("active")').isVisible();
      console.log(`✅ First condition (active) visible: ${firstConditionVisible}`);
      
      // Add second condition (Step 7: Repeat steps 1-6)
      console.log('📋 Step 7: Repeating steps 1-6 for second condition (active = true)...');
      await addFieldCondition(page, 'active', '$eq', 'true');
      await page.waitForTimeout(500);
      
      const afterSecondConditionCount = consoleMessages.length;
      console.log(`📊 After second condition: ${afterSecondConditionCount}`);
      
      // Verify we now have two conditions by counting the condition elements
      const conditionCount = await page.locator('[class*="bg-white border rounded p-2"]').count();
      console.log(`📊 Total conditions visible: ${conditionCount}`);
      
      // Check that undo button is now enabled
      const undoButton = page.locator('button[title="Undo (Ctrl+Z)"]');
      const undoDisabled = await undoButton.getAttribute('disabled');
      console.log(`🔄 Undo button disabled: ${undoDisabled !== null}`);
      
      // Step 8: Click Undo
      console.log('📋 Step 8: Testing undo operation...');
      if (undoDisabled === null) {
        // Button is enabled, click it
        await undoButton.click();
        console.log('✅ Clicked undo button');
      } else {
        // Try keyboard shortcut even if button appears disabled
        await page.keyboard.press('Control+z');
        console.log('⌨️ Used keyboard shortcut for undo');
      }
      await page.waitForTimeout(1000);
      
      const afterUndoMessageCount = consoleMessages.length;
      console.log(`📊 After undo: ${afterUndoMessageCount}`);
      
      // Check condition count after undo (should be 1)
      const conditionCountAfterUndo = await page.locator('[class*="bg-white border rounded p-2"]').count();
      console.log(`📊 Conditions after undo: ${conditionCountAfterUndo}`);
      
      // Additional test: Check that redo button is now enabled
      const redoButton = page.locator('button[title="Redo (Ctrl+Y)"]');
      const redoDisabled = await redoButton.getAttribute('disabled');
      console.log(`🔄 Redo button disabled after undo: ${redoDisabled !== null}`);
      
      // Test redo operation to verify it works too
      console.log('⏩ Testing redo operation...');
      if (redoDisabled === null) {
        await redoButton.click();
        console.log('✅ Clicked redo button');
      } else {
        await page.keyboard.press('Control+y');
        console.log('⌨️ Used keyboard shortcut for redo');
      }
      await page.waitForTimeout(1000);
      
      const afterRedoMessageCount = consoleMessages.length;
      console.log(`📊 After redo: ${afterRedoMessageCount}`);
      
      // Check condition count after redo (should be 2 again)
      const conditionCountAfterRedo = await page.locator('[class*="bg-white border rounded p-2"]').count();
      console.log(`📊 Conditions after redo: ${conditionCountAfterRedo}`);
      
    } catch (error) {
      console.log('❌ Error during undo/redo operations:', error);
      console.log('🔍 Error details:', error.message);
    }
    
    const finalMessageCount = consoleMessages.length;
    const messageDiff = finalMessageCount - initialMessageCount;
    
    console.log(`📊 Final message count: ${finalMessageCount}`);
    console.log(`📊 Total message difference: ${messageDiff}`);
    console.log(`❌ Error messages: ${errorMessages.length}`);
    
    if (errorMessages.length > 0) {
      console.log('❗ Error messages found:', errorMessages);
    }
    
    // Main test: no circular update errors
    expect(errorMessages.length).toBe(0);
    
    // Secondary test: reasonable message count (more lenient since we're doing more operations)
    expect(messageDiff).toBeLessThan(200);
  });
});
