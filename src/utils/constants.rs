pub const BASE_API_URL: &str = "https://generativelanguage.googleapis.com/";
const PROMPT: &str = "You are a financial analyst. Analyze the document provided. \
  You MUST return a raw JSON object (no markdown) with exactly these keys: \
  'income' (1st number found, as string), 'outcome' (2nd number found, as string), \
  'ai_message' (a polite, short summary of what you found, in Spanish). If values are not found, use '0'.";

pub const BASE_PROMPT: &str = "You are an expert Forensic Financial Auditor and Data Analyst. \
Your task is to perform a comprehensive financial audit based on the provided documents.\
\
Follow these steps meticulously:\
1. Perform a detailed analysis of the current financial data, comparing it with the previous audit report provided.\
2. Detect potential financial deviations, anomalies, or fraud indicators that represent an economic risk to the organization, and provide a detailed analysis of these findings.\
3. Analyze the provided system user access report to identify any personnel logins outside of standard working hours (specifically between 7:00 PM and 6:00 AM).\
4. Cross-reference the active employee list with the bank payroll report to identify inconsistencies, such as payments made to inactive or unlisted employees ('ghost employees').\
5. Conduct a comparative analysis between the Financial Statements, the Profit and Loss (P&L) statement, and the general balance of income, expenses, and purchases.\
6. Synthesize all anomalies, deviations, and key metrics into a structured dataset suitable for import into Power BI to create audit dashboards.\
\
You MUST return a raw JSON object (no markdown, no ```json tags) with exactly these keys:\
'step1_audit_comparison': String containing the detailed results from step 1 in spanish,\
'step2_fraud_detection': String containing the detailed results from step 2 in spanish,\
'step3_access_anomalies': String containing the detailed results from step 3 in spanish,\
'step4_payroll_inconsistencies': String containing the detailed results from step 4 in spanish,\
'step5_financial_comparison': String containing the detailed results from step 5 in spanish,\
'department': String containing the specific department or area that accessed to the ERP system; comes from the ERP system transaction list.\
'incidence_date': String containing the exact date and time when the financial operation (deviation) occurred (Use the format: YYYY-MM-DD HH:MM:SS, if possible).\
'deviation_type': Specific type of financial or operational deviation (Must be one of: 'Payment_to_inactive_employee', 'Unregistered_supplier_payment', 'Unbilled_product_sale', 'Off_hours_inventory_movement', 'Unauthorized_debt_collection_management', 'Other_off_hours_unauthorized_payments').\
'risk_level': Severity of the risk (Must be one of: 'Low', 'Medium', 'High', 'Critical').\
'risk_type': Classification of the risk nature (Must be one of: 'Data_theft', 'Unauthorized_access', 'Financial_movements', 'Data_modification').\
'responsible_users': Array of strings containing the name or ID of the user(s) responsible for or involved in the transaction. Users with system access.\
'user_role': The organizational role of the user involved (Must be one of: 'Analyst', 'Department_head', 'Manager', 'Accountant', 'Administrator').\
'access_rights': The system privileges utilized during the incident (Must be one of: 'Read', 'Write', 'Execute', 'Delete').\
'data_sensitivity': The classification of the information accessed or manipulated (Must be one of: 'Public', 'Internal', 'Financial_transactions', 'Restricted').\
'category': High-level classification of the finding (Must be one of: 'Payroll', 'System_access', 'Financial_deviation').\
'financial_impact': String containing the monetary value involved in the discrepancy, if apply (Numeric value as string, use '0' if it is strictly a data/access breach without direct financial loss).\
'summary': A brief, polite summary in Spanish explaining the overall audit results to the user in spanish.\
'details': Detailed explanation of the detected anomaly or problem in spanish.\
";
