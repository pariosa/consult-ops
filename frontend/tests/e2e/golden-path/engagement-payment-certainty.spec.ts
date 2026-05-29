import { test, expect } from '@playwright/test';

test('golden path: agreement + payout rules + milestone approval creates transaction trail', async ({
  page,
}) => {
  await page.goto('/test-login');

  await page.goto('/projects');

  await page.getByRole('button', { name: /new project/i }).click();
  await page.getByLabel(/project name/i).fill('Golden Path Project');
  await page.getByRole('button', { name: /create project/i }).click();

  await page.getByRole('button', { name: /new engagement/i }).click();
  await page.getByLabel(/title/i).fill('Website Launch Engagement');
  await page.getByRole('button', { name: /create engagement/i }).click();

  await page.getByRole('button', { name: /add milestone/i }).click();
  await page.getByLabel(/title/i).fill('Homepage delivery');
  await page.getByLabel(/amount/i).fill('1200');
  await page.getByRole('button', { name: /save milestone/i }).click();

  await page.getByRole('link', { name: /agreement/i }).click();

  await page
    .getByRole('button', { name: /create operational agreement/i })
    .click();

  await page.getByRole('button', { name: /add payout rule/i }).click();
  await page.getByLabel(/percent/i).fill('100');
  await page.getByLabel(/trigger/i).selectOption('milestone_approved');
  await page.getByRole('button', { name: /save payout rule/i }).click();

  await page.getByRole('button', { name: /lock agreement/i }).click();

  await page.getByRole('link', { name: /milestones/i }).click();

  await page.getByRole('button', { name: /approve milestone/i }).click();

  await expect(page.getByText(/transaction created/i)).toBeVisible();

  await page.getByRole('link', { name: /transactions/i }).click();

  await expect(page.getByText(/homepage delivery/i)).toBeVisible();
  await expect(page.getByText(/\$1,200|1200/)).toBeVisible();
  await expect(
    page.getByText(/milestone_approved|milestone approved/i),
  ).toBeVisible();

  await page.getByRole('link', { name: /timeline|audit trail/i }).click();

  await expect(page.getByText(/milestone approved/i)).toBeVisible();
  await expect(page.getByText(/transaction created/i)).toBeVisible();
  await expect(page.getByText(/agreement locked/i)).toBeVisible();
});
