//! Waveforms and conditional expressions used by signal assignments.
//!
//! A waveform is a chain of [`WaveformElement`] nodes (and possibly a trailing
//! [`UnaffectedWaveform`]). Conditional assignments wrap those waveforms or
//! expressions in [`ConditionalWaveform`] / [`ConditionalExpression`] chains.

use super::*;

/// One transaction in a signal-assignment waveform (`value [after time]`).
///
/// Scheduling uses [`value`](Self::value) as the projected waveform value and
/// optional [`delay`](Self::delay) as the `after` time expression. When
/// [`delay`](Self::delay) is absent, the assignment is a delta-cycle update
/// (delay 0).
///
/// ```vhdl
/// s <= '1';                      -- value = '1', delay = None (delta)
/// s <= '0' after 5 ns;            -- value = '0', delay = 5 ns
/// s <= transport '1' after 2 ns;  -- delay mechanism is on the assignment
/// s <= '1' after 1 ns, '0' after 2 ns;
/// -- two WaveformElement nodes in the waveforms chain
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct WaveformElement {
    /// Expression for the value driven by this transaction.
    #[serde(rename = "we_value")]
    pub value: ExpressionNodeId,
    /// Optional `after` time expression; absent means a delta delay.
    #[serde(rename = "time")]
    pub delay: Option<ExpressionNodeId>,
}

/// One arm of a conditional waveform assignment (`waveform when cond`).
///
/// Arms are chained; the final `else` arm has
/// [`condition`](Self::condition) = `None`.
///
/// ```vhdl
/// q <= a after 1 ns when sel = '1' else
///      b after 1 ns when en  = '1' else
///      '0';
/// -- three ConditionalWaveform nodes:
/// --   condition = sel='1', waveforms = [a after 1 ns]
/// --   condition = en='1',  waveforms = [b after 1 ns]
/// --   condition = None,    waveforms = ['0']
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionalWaveform {
    /// Condition of this arm; absent for the final `else` waveform.
    pub condition: Option<ExpressionNodeId>,
    /// Waveform elements driven when this arm is selected.
    #[serde(default)]
    pub waveforms: Vec<NodeId<WaveformElement>>,
}

/// The `unaffected` reserved word in a waveform.
///
/// Stands alone in the waveform chain and means the driver does not schedule a
/// new transaction (the projected waveform is unchanged). Legal in conditional
/// and selected assignments.
///
/// ```vhdl
/// q <= a when en = '1' else unaffected;
///
/// with sel select
///   q <= a when "00",
///        unaffected when others;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct UnaffectedWaveform {}

/// One arm of a conditional expression / conditional variable assignment.
///
/// VHDL-2008 conditional expressions and sequential conditional variable
/// assignments use this chain. The final `else` arm has
/// [`condition`](Self::condition) = `None`.
///
/// ```vhdl
/// y := a when sel = '1' else b when en = '1' else '0';
/// -- three ConditionalExpression nodes
///
/// -- Concurrent conditional force uses expression chains similarly:
/// -- (conditional signal force is modeled on the assignment node)
/// constant c : bit := '1' when RESET else '0';
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionalExpression {
    /// Condition of this arm; absent for the final `else` expression.
    pub condition: Option<ExpressionNodeId>,
    /// Expression value of this arm.
    pub expression: ExpressionNodeId,
}
