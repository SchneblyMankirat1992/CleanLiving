// Health and wellness consultancy module

mod health_wellness {
  // Structs
  struct Consultant {
    name: String,
    consult_time: u32,
  }
  
  // Enums
  enum FoodOption {
    Natural,
    Local,
    Organic,
  }
  
  // Functions
  fn schedule_consultation(name: &str, consult_time: u32) -> Consultant {
    Consultant {
      name: String::from(name),
      consult_time,
    }
  }
  
  fn recommend_food_options(food: FoodOption, num_options: u32) -> Vec<String> {
    let mut food_options = vec![];
    for _ in 0..num_options {
      match food {
        FoodOption::Natural => food_options.push("eat plenty of fruits and vegetables".to_string()),
        FoodOption::Local => food_options.push("buy from local farmers markets".to_string()),
        FoodOption::Organic => food_options.push("consume organic produce".to_string()),
      }
    }
    food_options
  }
  
  fn develop_strategy(consultant: &Consultant, food_options: &Vec<String>) -> String {
    format!("{} recommends that you {}", consultant.name, food_options.join(", "))
  }
  
  // Tests
  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_schedule_consultation() {
      let consultant = schedule_consultation("John Smith", 1);
      assert_eq!(consultant.name, "John Smith");
      assert_eq!(consultant.consult_time, 1);
    }
    
    #[test]
    fn test_recommend_food_options() {
      let natural_food_options = recommend_food_options(FoodOption::Natural, 2);
      assert_eq!(natural_food_options[0], "eat plenty of fruits and vegetables");
      assert_eq!(natural_food_options[1], "eat plenty of fruits and vegetables");
      
      let local_food_options = recommend_food_options(FoodOption::Local, 2);
      assert_eq!(local_food_options[0], "buy from local farmers markets");
      assert_eq!(local_food_options[1], "buy from local farmers markets");
      
      let organic_food_options = recommend_food_options(FoodOption::Organic, 2);
      assert_eq!(organic_food_options[0], "consume organic produce");
      assert_eq!(organic_food_options[1], "consume organic produce");
    }
    
    #[test]
    fn test_develop_strategy() {
      let consultant = schedule_consultation("John Smith", 1);
      let food_options = recommend_food_options(FoodOption::Organic, 2);
      let strategy = develop_strategy(&consultant, &food_options);
      assert_eq!(strategy, "John Smith recommends that you consume organic produce, consume organic produce");
    }
  }
}