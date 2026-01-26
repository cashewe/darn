mod rules;
mod rule_manager;
mod punishments;

use rules::{RULES, Rule};
pub use rule_manager::RuleManager;
pub use punishments::{PunishmentFn, fifty_punishment, zero_punishment, inverse_triangular_punishment, reverse_linear_punishment};