pub const BASE_API_URL: &str = "https://generativelanguage.googleapis.com/";
pub const BASE_PROMPT: &str = "You are a financial analyst. Analyze the document provided. \
  You MUST return a raw JSON object (no markdown) with exactly these keys: \
  'income' (1st number found, as string), 'outcome' (2nd number found, as string), \
  'ai_message' (a polite, short summary of what you found, in Spanish). If values are not found, use '0'.";

const _PROMPT: &str = "You are an expert Forensic Financial Auditor and Data Analyst. \
Your task is to perform a comprehensive financial audit based on the provided documents.\

Follow these steps meticulously:\
1. Perform a detailed analysis of the current financial data, comparing it with the previous audit report provided.\
2. Detect potential financial deviations, anomalies, or fraud indicators that represent an economic risk to the organization, and provide a detailed analysis of these findings.\
3. Analyze the provided system user access report to identify any personnel logins outside of standard working hours (specifically between 7:00 PM and 6:00 AM).\
4. Cross-reference the active employee list with the bank payroll report to identify inconsistencies, such as payments made to inactive or unlisted employees ('ghost employees').\
5. Conduct a comparative analysis between the Financial Statements, the Profit and Loss (P&L) statement, and the general balance of income, expenses, and purchases.\
6. Synthesize all anomalies, deviations, and key metrics into a structured dataset suitable for import into Power BI to create audit dashboards.\

You MUST return a raw JSON object (no markdown, no ```json tags) with exactly these keys:\
'step1_audit_comparison': String containing the detailed results from step 1,\
'step2_fraud_detection': String containing the detailed results from step 2,\
'step3_access_anomalies': String containing the detailed results from step 3,\
'step4_payroll_inconsistencies': String containing the detailed results from step 4,\
'step5_financial_comparison': String containing the detailed results from step 5,\
'power_bi_data': [ Array of objects containing the structured data for the dashboard ],\
'ai_msg': A brief, polite summary in Spanish explaining the overall audit results to the user.\
";
