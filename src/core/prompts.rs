use crate::modules::profiles::model::TargetsResponse;

pub const SCAN_SYSTEM_PROMPT: &str = "You are a nutritional auditor. Your task is to reconcile visual data with user claims. Output ONLY valid JSON.";

pub const SCAN_USER_PROMPT: &str = r#"Perform a nutritional audit. Visual data identifies the food, but TEXT INPUT IS THE AUTHORITY on quantity.

__USER_ADDITIONAL_TEXT_INPUT__




__TARGET_CONTEXT__


Output ONLY valid JSON, format:
{
  "name": "Short food name (max 3 words)",
  "description": "Description of the food",
  "meal_type": string, (Breakfast, Lunch, Dinner, Snack)
  "calories": float,
  "protein": float,
  "carbs": float,
  "fats": float,
  "fiber": float,
  "confidence": float (0.0–1.0),
  "items": ["e.g. 3 roti", "1 bowl dal"],
  "reasoning": "Brief explanation of totals based on image + TEXT_INPUT"
}
items MUST reflect text TEXT_INPUT adjustments. Macros MUST match items."#;

pub const TARGET_CONTEXT: &str = r#"
USER'S DAILY TARGETS:
- Calories: __CALORIES__ kcal
- Protein: __PROTEIN__ g
- Carbs: __CARBS__ g
- Fat: __FAT__ g
 
TARGETS RULES:
- Note "HighProtein" in reasoning if protein > 30% of target
- Note "Substantial Meal" in reasoning if calories > 40% of daily target
"#;

const USER_ADDITIONAL_TEXT_INPUT: &str = r#"USER ADDITIONAL TEXT INPUT (along with image): __LABEL__

TEXT_INPUT RULES:
1. Quantity Override: If text says "ate 3" or "only 1", use that number regardless of image.
2. Ingredient Override: If text mentions "butter", "oil", or specific ingredients, adjust accordingly.
3. Visual Fallback: Use image for quantity ONLY if text does not specify it.
"#;

pub fn build_user_context(target: &TargetsResponse) -> String {
    TARGET_CONTEXT
        .replace("__CALORIES__", &target.target_calories.to_string())
        .replace("__PROTEIN__", &target.target_protein.to_string())
        .replace("__CARBS__", &target.target_carbs.to_string())
        .replace("__FAT__", &target.target_fats.to_string())
}

pub fn build_user_scan_prompt(label: Option<&str>, target: Option<&TargetsResponse>) -> String {
    let label_context = label
        .map(|l| USER_ADDITIONAL_TEXT_INPUT.replace("__LABEL__", l))
        .unwrap_or_default();

    let target_context = target.map(|t| build_user_context(t)).unwrap_or_default();

    SCAN_USER_PROMPT
        .replace("__USER_ADDITIONAL_TEXT_INPUT__", &label_context)
        .replace("__TARGET_CONTEXT__", &target_context)
}
