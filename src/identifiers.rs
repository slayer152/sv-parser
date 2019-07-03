use crate::expressions::*;
use crate::primaries::*;
use crate::util::*;
use nom::branch::*;
use nom::bytes::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

const AZ_: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
const AZ09_: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
const AZ09_DOLLAR: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_$";

#[derive(Debug)]
pub struct Identifier<'a> {
    pub raw: Vec<&'a str>,
}

#[derive(Debug)]
pub struct ScopedIdentifier<'a> {
    pub scope: Option<Scope<'a>>,
    pub identifier: HierarchicalIdentifier<'a>,
}

#[derive(Debug)]
pub enum Scope<'a> {
    LocalScope,
    PackageScope(Identifier<'a>),
    ClassScope,
    ImplicitClassHandle(ImplicitClassHandle),
    GenerateBlockScope(Vec<GenerateBlockScope<'a>>),
}

#[derive(Debug)]
pub struct GenerateBlockScope<'a> {
    pub identifier: Identifier<'a>,
    pub constant_expression: Option<ConstantExpression<'a>>,
}

#[derive(Debug)]
pub struct HierarchicalIdentifier<'a> {
    pub hierarchy: Vec<Hierarchy<'a>>,
    pub identifier: Identifier<'a>,
}

impl<'a> From<Identifier<'a>> for HierarchicalIdentifier<'a> {
    fn from(x: Identifier<'a>) -> Self {
        HierarchicalIdentifier {
            hierarchy: vec![],
            identifier: x,
        }
    }
}

#[derive(Debug)]
pub struct Hierarchy<'a> {
    pub identifier: Identifier<'a>,
    pub constant_bit_select: Option<Vec<ConstantExpression<'a>>>,
}

// -----------------------------------------------------------------------------

pub fn array_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn block_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn bin_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn c_identifier(s: &str) -> IResult<&str, Identifier> {
    let (s, x) = is_a(AZ_)(s)?;
    let (s, y) = opt(is_a(AZ09_))(s)?;
    let raw = if let Some(y) = y { vec![x, y] } else { vec![x] };
    Ok((s, Identifier { raw }))
}

pub fn cell_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn checker_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn class_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn class_variable_identifier(s: &str) -> IResult<&str, Identifier> {
    variable_identifier(s)
}

pub fn clocking_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn config_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn const_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn constraint_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn covergroup_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn covergroup_variable_identifier(s: &str) -> IResult<&str, Identifier> {
    variable_identifier(s)
}

pub fn cover_point_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn cross_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn dynamic_array_variable_identifier(s: &str) -> IResult<&str, Identifier> {
    variable_identifier(s)
}

pub fn enum_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn escaped_identifier(s: &str) -> IResult<&str, Identifier> {
    let (s, x) = tag("\\")(s)?;
    let (s, y) = is_not(" \t\r\n")(s)?;
    Ok((s, Identifier { raw: vec![x, y] }))
}

pub fn formal_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn formal_port_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn function_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn generate_block_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn genvar_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn hierarchical_array_identifier(s: &str) -> IResult<&str, HierarchicalIdentifier> {
    hierarchical_identifier(s)
}

pub fn hierarchical_block_identifier(s: &str) -> IResult<&str, HierarchicalIdentifier> {
    hierarchical_identifier(s)
}

pub fn hierarchical_event_identifier(s: &str) -> IResult<&str, HierarchicalIdentifier> {
    hierarchical_identifier(s)
}

pub fn hierarchy(s: &str) -> IResult<&str, Hierarchy> {
    let (s, identifier) = identifier(s)?;
    let (s, constant_bit_select) = sp(constant_bit_select)(s)?;
    let (s, _) = sp(tag("."))(s)?;

    let constant_bit_select = Some(constant_bit_select);

    Ok((
        s,
        Hierarchy {
            identifier,
            constant_bit_select,
        },
    ))
}

pub fn hierarchical_identifier(s: &str) -> IResult<&str, HierarchicalIdentifier> {
    let (s, x) = opt(terminated(tag("$root"), sp(tag("."))))(s)?;
    let (s, mut hierarchy) = many0(hierarchy)(s)?;
    let (s, identifier) = sp(identifier)(s)?;

    if let Some(x) = x {
        hierarchy.insert(
            0,
            Hierarchy {
                identifier: Identifier { raw: vec![x] },
                constant_bit_select: None,
            },
        );
    }

    Ok((
        s,
        HierarchicalIdentifier {
            hierarchy,
            identifier,
        },
    ))
}

pub fn hierarchical_net_identifier(s: &str) -> IResult<&str, HierarchicalIdentifier> {
    hierarchical_identifier(s)
}

pub fn hierarchical_parameter_identifier(s: &str) -> IResult<&str, HierarchicalIdentifier> {
    hierarchical_identifier(s)
}

pub fn hierarchical_property_identifier(s: &str) -> IResult<&str, HierarchicalIdentifier> {
    hierarchical_identifier(s)
}

pub fn hierarchical_sequence_identifier(s: &str) -> IResult<&str, HierarchicalIdentifier> {
    hierarchical_identifier(s)
}

pub fn hierarchical_task_identifier(s: &str) -> IResult<&str, HierarchicalIdentifier> {
    hierarchical_identifier(s)
}

pub fn hierarchical_tf_identifier(s: &str) -> IResult<&str, HierarchicalIdentifier> {
    hierarchical_identifier(s)
}

pub fn hierarchical_variable_identifier(s: &str) -> IResult<&str, HierarchicalIdentifier> {
    hierarchical_identifier(s)
}

pub fn identifier(s: &str) -> IResult<&str, Identifier> {
    alt((escaped_identifier, simple_identifier))(s)
}

pub fn index_variable_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn interface_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn interface_instance_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn inout_port_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn input_port_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn instance_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn library_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn member_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn method_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn modport_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn module_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn net_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn output_port_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn package_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn package_scope(s: &str) -> IResult<&str, Scope> {
    let (s, x) = alt((
        terminated(package_identifier, sp(tag("::"))),
        terminated(
            map(tag("$unit"), |x| Identifier { raw: vec![x] }),
            sp(tag("::")),
        ),
    ))(s)?;
    Ok((s, Scope::PackageScope(x)))
}

pub fn parameter_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn port_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn production_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn program_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn property_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn ps_class_identifier(s: &str) -> IResult<&str, ScopedIdentifier> {
    let (s, scope) = opt(package_scope)(s)?;
    let (s, identifier) = sp(class_identifier)(s)?;
    let identifier = identifier.into();
    Ok((s, ScopedIdentifier { scope, identifier }))
}

pub fn ps_covergroup_identifier(s: &str) -> IResult<&str, ScopedIdentifier> {
    let (s, scope) = opt(package_scope)(s)?;
    let (s, identifier) = sp(covergroup_identifier)(s)?;
    let identifier = identifier.into();
    Ok((s, ScopedIdentifier { scope, identifier }))
}

pub fn ps_checker_identifier(s: &str) -> IResult<&str, ScopedIdentifier> {
    let (s, scope) = opt(package_scope)(s)?;
    let (s, identifier) = sp(checker_identifier)(s)?;
    let identifier = identifier.into();
    Ok((s, ScopedIdentifier { scope, identifier }))
}

pub fn ps_identifier(s: &str) -> IResult<&str, ScopedIdentifier> {
    let (s, scope) = opt(package_scope)(s)?;
    let (s, identifier) = sp(identifier)(s)?;
    let identifier = identifier.into();
    Ok((s, ScopedIdentifier { scope, identifier }))
}

pub fn ps_or_hierarchical_array_identifier(s: &str) -> IResult<&str, ScopedIdentifier> {
    let (s, scope) = opt(alt((
        terminated(implicit_class_handle, sp(tag("."))),
        class_scope,
        package_scope,
    )))(s)?;
    let (s, identifier) = sp(hierarchical_array_identifier)(s)?;
    Ok((s, ScopedIdentifier { scope, identifier }))
}

pub fn ps_or_hierarchical_net_identifier(s: &str) -> IResult<&str, ScopedIdentifier> {
    let (s, scope) = opt(package_scope)(s)?;
    let (s, identifier) = alt((
        map(sp(net_identifier), |x| x.into()),
        sp(hierarchical_net_identifier),
    ))(s)?;
    Ok((s, ScopedIdentifier { scope, identifier }))
}

pub fn ps_or_hierarchical_property_identifier(s: &str) -> IResult<&str, ScopedIdentifier> {
    let (s, scope) = opt(package_scope)(s)?;
    let (s, identifier) = alt((
        map(sp(property_identifier), |x| x.into()),
        sp(hierarchical_property_identifier),
    ))(s)?;
    Ok((s, ScopedIdentifier { scope, identifier }))
}

pub fn ps_or_hierarchical_sequence_identifier(s: &str) -> IResult<&str, ScopedIdentifier> {
    let (s, scope) = opt(package_scope)(s)?;
    let (s, identifier) = alt((
        map(sp(sequence_identifier), |x| x.into()),
        sp(hierarchical_sequence_identifier),
    ))(s)?;
    Ok((s, ScopedIdentifier { scope, identifier }))
}

pub fn ps_or_hierarchical_tf_identifier(s: &str) -> IResult<&str, ScopedIdentifier> {
    let (s, scope) = opt(package_scope)(s)?;
    let (s, identifier) = alt((
        map(sp(tf_identifier), |x| x.into()),
        sp(hierarchical_tf_identifier),
    ))(s)?;
    Ok((s, ScopedIdentifier { scope, identifier }))
}

pub fn ps_parameter_identifier(s: &str) -> IResult<&str, ScopedIdentifier> {
    let (s, scope) = opt(alt((package_scope, class_scope, generate_block_scope)))(s)?;
    let (s, identifier) = sp(parameter_identifier)(s)?;
    let identifier = identifier.into();
    Ok((s, ScopedIdentifier { scope, identifier }))
}

pub fn generate_block_scope(s: &str) -> IResult<&str, Scope> {
    let (s, x) = many0(tuple((
        sp(generate_block_identifier),
        opt(delimited(
            sp(tag("[")),
            sp(constant_expression),
            sp(tag("]")),
        )),
        sp(tag(".")),
    )))(s)?;

    let mut ret = Vec::new();
    for (identifier, constant_expression, _) in x {
        ret.push(GenerateBlockScope {
            identifier,
            constant_expression,
        });
    }

    Ok((s, Scope::GenerateBlockScope(ret)))
}

pub fn ps_type_identifier(s: &str) -> IResult<&str, ScopedIdentifier> {
    let (s, scope) = opt(alt((
        map(terminated(tag("local"), sp(tag("::"))), |_| {
            Scope::LocalScope
        }),
        package_scope,
        class_scope,
    )))(s)?;
    let (s, identifier) = sp(type_identifier)(s)?;
    let identifier = identifier.into();
    Ok((s, ScopedIdentifier { scope, identifier }))
}

pub fn sequence_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn signal_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn simple_identifier(s: &str) -> IResult<&str, Identifier> {
    let (s, x) = is_a(AZ_)(s)?;
    let (s, y) = opt(is_a(AZ09_DOLLAR))(s)?;
    let raw = if let Some(y) = y { vec![x, y] } else { vec![x] };
    Ok((s, Identifier { raw }))
}

pub fn specparam_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn system_tf_identifier(s: &str) -> IResult<&str, Identifier> {
    let (s, x) = tag("$")(s)?;
    let (s, y) = is_a(AZ09_DOLLAR)(s)?;
    Ok((s, Identifier { raw: vec![x, y] }))
}

pub fn task_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn tf_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn terminal_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn topmodule_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn type_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn udp_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

pub fn variable_identifier(s: &str) -> IResult<&str, Identifier> {
    identifier(s)
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            format!("{:?}", all_consuming(identifier)("shiftreg_a")),
            "Ok((\"\", Identifier { raw: [\"shiftreg_a\"] }))"
        );
        assert_eq!(
            format!("{:?}", all_consuming(identifier)("_bus3")),
            "Ok((\"\", Identifier { raw: [\"_bus\", \"3\"] }))"
        );
        assert_eq!(
            format!("{:?}", all_consuming(identifier)("n$657")),
            "Ok((\"\", Identifier { raw: [\"n\", \"$657\"] }))"
        );
        assert_eq!(
            format!("{:?}", all_consuming(identifier)("\\busa+index")),
            "Ok((\"\", Identifier { raw: [\"\\\\\", \"busa+index\"] }))"
        );
        assert_eq!(
            format!("{:?}", all_consuming(identifier)("\\-clock")),
            "Ok((\"\", Identifier { raw: [\"\\\\\", \"-clock\"] }))"
        );
        assert_eq!(
            format!("{:?}", all_consuming(system_tf_identifier)("$display")),
            "Ok((\"\", Identifier { raw: [\"$\", \"display\"] }))"
        );
    }
}