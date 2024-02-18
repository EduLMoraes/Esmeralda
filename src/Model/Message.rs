/// Represents a message with a hidden flag and text content.
///
/// # Fields
///
/// - `hidden`: A boolean indicating whether the message is hidden or not.
/// - `text`: A reference to a string representing the content of the message.
///
/// # Example
///
/// ```
/// struct Message<'a> {
///     hidden: bool,
///     text: &'a str,
/// }
/// ```
pub struct Message<'a> {
    pub hidden: bool,
    pub text: &'a str,
}