use super::*;

subset_declaration!(Name NameNodeId {
    AttributeName(AttributeName),
    IndexedName(IndexedName),
    SelectedByAllName(SelectedByAllName),
    SelectedName(SelectedName),
    SimpleName(SimpleName),
    SliceName(SliceName),
});

impl Name<'_> {
    #[must_use]
    pub fn named_entity(&self) -> Option<NamedEntityNodeId> {
        match self {
            Self::AttributeName(attribute_name) => Some(attribute_name.named_entity),
            Self::SelectedName(selected_name) => Some(selected_name.named_entity),
            Self::SimpleName(simple_name) => Some(simple_name.named_entity),
            Self::IndexedName(_) | Self::SelectedByAllName(_) | Self::SliceName(_) => None,
        }
    }

    #[must_use]
    pub fn elements(&self, ast: &Ast) -> NameElements {
        let mut elements = Vec::with_capacity(4);

        let mut next_prefix = match self {
            Self::AttributeName(attribute_name) => {
                elements.push(NameElement::NamedEntity(attribute_name.named_entity));
                attribute_name.prefix
            },
            Self::IndexedName(indexed_name) => {
                elements.push(NameElement::Other);
                indexed_name.prefix
            },
            Self::SelectedByAllName(selected_by_all_name) => {
                elements.push(NameElement::All);
                selected_by_all_name.prefix
            },
            Self::SelectedName(selected_name) => {
                elements.push(NameElement::NamedEntity(selected_name.named_entity));
                selected_name.prefix
            },
            Self::SimpleName(simple_name) => {
                elements.push(NameElement::NamedEntity(simple_name.named_entity));
                return NameElements { elements };
            },
            Self::SliceName(slice_name) => {
                elements.push(NameElement::Other);
                slice_name.prefix
            },
        };

        loop {
            next_prefix = match next_prefix.get(ast) {
                Prefix::AttributeName(attribute_name) => {
                    elements.push(NameElement::NamedEntity(attribute_name.named_entity));
                    attribute_name.prefix
                },
                Prefix::IndexedName(indexed_name) => {
                    elements.push(NameElement::Other);
                    indexed_name.prefix
                },
                Prefix::SelectedName(selected_name) => {
                    elements.push(NameElement::NamedEntity(selected_name.named_entity));
                    selected_name.prefix
                },
                Prefix::SimpleName(simple_name) => {
                    elements.push(NameElement::NamedEntity(simple_name.named_entity));
                    return NameElements { elements };
                },
                Prefix::SliceName(slice_name) => {
                    elements.push(NameElement::Other);
                    slice_name.prefix
                },

                Prefix::SelectedElement(selected_element) => {
                    elements.push(NameElement::NamedEntity(selected_element.named_entity));
                    selected_element.prefix
                },
                Prefix::FunctionCall(function_call) => {
                    elements.push(NameElement::Other);
                    function_call.prefix
                },
                Prefix::Dereference(dereference) => {
                    // Dereference operation, not an element of the name
                    dereference.prefix
                },
                Prefix::ImplicitDereference(implicit_dereference) => {
                    // Implicit operation, not an element of the name
                    implicit_dereference.prefix
                },
                Prefix::OperatorSymbol(_) => {
                    elements.push(NameElement::Other);
                    return NameElements { elements };
                },
            };
        }
    }
}

pub struct NameElements {
    /// The elements of the name, in reverse order.
    elements: Vec<NameElement>,
}

impl NameElements {
    #[must_use]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = NameElement> + ExactSizeIterator {
        self.elements.iter().copied().rev()
    }

    #[must_use]
    pub fn named_entities(&self) -> impl DoubleEndedIterator<Item = NamedEntityNodeId> {
        self.elements
            .iter()
            .rev()
            .filter_map(|element| match element {
                &NameElement::NamedEntity(named_entity) => Some(named_entity),
                _ => None,
            })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum NameElement {
    NamedEntity(NamedEntityNodeId),
    All,
    Other,
}

subset_declaration!(AnySelectedName AnySelectedNameNodeId {
    SelectedName(SelectedName),
    SelectedByAllName(SelectedByAllName),
});

impl<'node> From<AnySelectedName<'node>> for Name<'node> {
    fn from(value: AnySelectedName<'node>) -> Self {
        match value {
            AnySelectedName::SelectedName(selected_name) => Name::SelectedName(selected_name),
            AnySelectedName::SelectedByAllName(selected_by_all_name) => {
                Name::SelectedByAllName(selected_by_all_name)
            },
        }
    }
}

subset_declaration!(Prefix PrefixNodeId {
    AttributeName(AttributeName),
    IndexedName(IndexedName),
    SelectedName(SelectedName),
    SimpleName(SimpleName),
    SliceName(SliceName),

    SelectedElement(SelectedElement),
    FunctionCall(SubprogramCall),
    Dereference(Dereference),
    ImplicitDereference(ImplicitDereference),
    OperatorSymbol(OperatorSymbol),
});

impl Prefix<'_> {
    #[must_use]
    pub fn named_entity(&self) -> Option<NamedEntityNodeId> {
        match self {
            Self::AttributeName(attribute_name) => Some(attribute_name.named_entity),
            Self::SelectedName(selected_name) => Some(selected_name.named_entity),
            Self::SimpleName(simple_name) => Some(simple_name.named_entity),
            Self::IndexedName(_) | Self::SliceName(_) => None,

            _ => todo!(),
        }
    }
}

/// ```text
/// prefix: &simple_name
/// type: &physical_type_definition | &array_subtype_definition | &record_type_definition | &floating_type_definition | &enumeration_type_definition | &integer_type_definition | &integer_subtype_definition
/// identifier: "…"
/// base_name: &attribute_value
/// named_entity: &attribute_value
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeName {
    pub prefix: PrefixNodeId,
    pub named_entity: NamedEntityNodeId,
}

/// ```text
/// prefix: &simple_name
/// type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &physical_subtype_definition | &array_type_definition | &integer_subtype_definition
/// base_name: &dereference
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct Dereference {
    pub prefix: PrefixNodeId,
}

/// ```text
/// prefix: &simple_name
/// base_name: &implicit_dereference
/// type: &array_subtype_definition | &record_type_definition | &array_type_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ImplicitDereference {
    pub prefix: PrefixNodeId,
}

/// ```text
/// index_list: "others" | &[character_literal] | &[indexed_name] | &[integer_literal] | &[right_array_attribute] | &[left_array_attribute] | &[enumeration_literal] | &[substraction_operator] | &[addition_operator] | &[simple_name] | &[selected_element]
/// prefix: &indexed_name | &slice_name | &function_call | &selected_element | &implicit_dereference | &simple_name | &attribute_name
/// type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &record_subtype_definition | &physical_subtype_definition | &integer_subtype_definition
/// base_name: &signal_declaration | &interface_signal_declaration | &function_call | &for_generate_statement | &interface_variable_declaration | &implicit_dereference | &variable_declaration | &object_alias_declaration | &attribute_value | &constant_declaration | &interface_constant_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IndexedName {
    pub prefix: PrefixNodeId,
    pub index_list: IndexList,
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
}

/// ```text
/// base_name: &function_declaration
/// identifier: "and" | "mod" | "+" | "abs"
/// named_entity: &function_declaration | &function_body
/// type: &enumeration_type_definition
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct OperatorSymbol {}

/// ```text
/// base_name: &signal_declaration | &interface_signal_declaration | &function_call | &interface_variable_declaration | &implicit_dereference | &variable_declaration | &attribute_value | &constant_declaration | &interface_constant_declaration
/// type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &physical_subtype_definition | &integer_subtype_definition
/// identifier: "…"
/// named_entity: &element_declaration
/// prefix: &selected_element | &implicit_dereference | &indexed_name | &function_call | &simple_name | &attribute_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedElement {
    pub prefix: PrefixNodeId,
    pub named_entity: NamedEntityNodeId,
}

/// ```text
/// prefix: &simple_name | &selected_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedByAllName {
    pub prefix: PrefixNodeId,
}

/// ```text
/// named_entity: &iterator_declaration | &interface_signal_declaration | &entity_declaration | &function_declaration | &subtype_declaration | &type_declaration | &package_declaration | &configuration_declaration | &variable_declaration | &procedure_declaration | &constant_declaration | &signal_declaration
/// prefix: &selected_name | &simple_name | &operator_symbol
/// type: &enumeration_subtype_definition | &record_type_definition | &array_subtype_definition | &enumeration_type_definition | &physical_subtype_definition | &array_type_definition | &integer_subtype_definition
/// base_name: &signal_declaration | &interface_signal_declaration | &subtype_declaration | &type_declaration | &variable_declaration | &constant_declaration | &iterator_declaration
/// identifier: "…"
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedName {
    pub identifier: Identifier,
    pub named_entity: NamedEntityNodeId,
    pub prefix: PrefixNodeId,
}

/// ```text
/// base_name: &interface_signal_declaration | &attribute_declaration | &package_declaration | &architecture_body | &configuration_declaration | &object_alias_declaration | &iterator_declaration | &interface_file_declaration | &interface_constant_declaration | &while_loop_statement | &type_declaration | &if_generate_statement | &interface_variable_declaration | &file_declaration | &for_loop_statement | &enumeration_literal | &variable_declaration | &library_declaration | &simple_name | &constant_declaration | &signal_declaration | &entity_declaration | &component_declaration | &for_generate_statement | &subtype_declaration | &function_declaration | &guard_signal_declaration | &block_statement | &procedure_declaration | &unit_declaration
/// type: &floating_subtype_definition | &physical_type_definition | &array_subtype_definition | &physical_subtype_definition | &access_subtype_definition | &integer_subtype_definition | &array_type_definition | &access_type_definition | &enumeration_subtype_definition | &record_type_definition | &incomplete_type_definition | &record_subtype_definition | &enumeration_type_definition | &file_type_definition
/// identifier: "…"
/// named_entity: &interface_signal_declaration | &attribute_declaration | &package_declaration | &architecture_body | &configuration_declaration | &object_alias_declaration | &iterator_declaration | &interface_file_declaration | &interface_constant_declaration | &while_loop_statement | &type_declaration | &interface_variable_declaration | &file_declaration | &element_declaration | &for_loop_statement | &process_statement | &enumeration_literal | &variable_declaration | &library_declaration | &constant_declaration | &entity_declaration | &signal_declaration | &component_declaration | &subtype_declaration | &block_statement | &guard_signal_declaration | &function_declaration | &component_instantiation_statement | &procedure_declaration | &generate_statement_body | &unit_declaration
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleName {
    pub identifier: Identifier,
    #[serde(default = "unresolved_named_entity")]
    pub named_entity: NamedEntityNodeId,
}

fn unresolved_named_entity() -> NamedEntityNodeId {
    Error::GLOBAL_ID.into()
}

/// ```text
/// prefix: &indexed_name | &slice_name | &function_call | &selected_element | &implicit_dereference | &simple_name | &attribute_name
/// slice_subtype: &array_subtype_definition
/// suffix: &range_expression | &range_array_attribute | &reverse_range_array_attribute
/// type: &array_subtype_definition
/// base_name: &interface_constant_declaration | &function_call | &implicit_dereference | &variable_declaration | &object_alias_declaration | &attribute_value | &constant_declaration | &signal_declaration
#[derive(Debug, Deserialize, Serialize)]
pub struct SliceName {
    pub prefix: PrefixNodeId,
    pub suffix: RangeConstraintNodeId,
}

subset_declaration!(NamedEntity NamedEntityNodeId {
    TypeDeclaration(TypeDeclaration),
    VariableDeclaration(VariableDeclaration),
    ConstantDeclaration(ConstantDeclaration),
    SignalDeclaration(SignalDeclaration),
    FileDeclaration(FileDeclaration),

    // InterfaceTypeDeclaration(InterfaceTypeDeclaration),
    // InterfaceVariableDeclaration(InterfaceVariableDeclaration),
    InterfaceConstantDeclaration(InterfaceConstantDeclaration),
    InterfaceSignalDeclaration(InterfaceSignalDeclaration),
    InterfaceFileDeclaration(InterfaceFileDeclaration),

    // TODO "An alias declaration" (LRM § 6.1)

    AttributeDeclaration(AttributeDeclaration),
    ComponentDeclaration(ComponentDeclaration),
    // TODO "A group template declaration" (LRM § 6.1)
    // TODO "A group declaration" (LRM § 6.1)
    SubprogramDeclaration(SubprogramDeclaration),
    // TODO: "A subprogram instantiation declaration" (LRM § 6.1)
    ConfigurationDeclaration(ConfigurationDeclaration),
    ContextDeclaration(ContextDeclaration),
    EntityDeclaration(EntityDeclaration),
    PackageDeclaration(PackageDeclaration),
    PackageInstantiationDeclaration(PackageInstantiationDeclaration),
    ArchitectureBody(ArchitectureBody),
    // TODO "A PSL property declaration" (LRM § 6.1)
    // TODO "A PSL sequence declaration" (LRM § 6.1)
    EnumerationLiteral(EnumerationLiteral),
    UnitDeclaration(UnitDeclaration),
    ElementDeclaration(ElementDeclaration),
    // TODO "A parameter specification in a loop statement or a for generate statement" (LRM § 6.1)
    // TODO "An implicit label declaration" (LRM § 6.1)
    Library(Library),
    /// Dummy node for unresolved names.
    ///
    /// There should only be one case where this variant occurs: as the named entity of a
    /// `SimpleName` for the explicit architecture body of a component instantiation, and no
    /// architecture of that name exists:
    ///
    /// ```vhdl
    /// mult: entity work.Mult(parallel) ...
    /// ```
    Unresolved(Error),
});
