use super::*;

subset_declaration!(DenotingName DenotingNameNodeId {
    CharacterLiteral(CharacterLiteral),
    SimpleName(SimpleName),
    SelectedName(SelectedName),
    OperatorSymbol(OperatorSymbol),
    ReferenceName(ReferenceName),
});

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
    FunctionCall(FunctionCall),
    Dereference(Dereference),
    ImplicitDereference(ImplicitDereference),
    OperatorSymbol(OperatorSymbol),
});

#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeName {
    pub prefix: PrefixNodeId,
    pub named_entity: NamedEntityNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dereference {
    pub prefix: PrefixNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImplicitDereference {
    pub prefix: PrefixNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexedName {
    pub prefix: PrefixNodeId,
    pub index_list: IndexList,
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OperatorSymbol {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedElement {
    pub prefix: PrefixNodeId,
    pub named_entity: NamedEntityNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedByAllName {
    pub prefix: PrefixNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedName {
    pub identifier: Identifier,
    pub named_entity: NamedEntityNodeId,
    pub prefix: PrefixNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleName {
    pub identifier: Identifier,
    #[serde(default = "unresolved_named_entity")]
    pub named_entity: NamedEntityNodeId,
}

fn unresolved_named_entity() -> NamedEntityNodeId {
    Error::GLOBAL_ID.into()
}

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
    FunctionDeclaration(FunctionDeclaration),
    ProcedureDeclaration(ProcedureDeclaration),
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ReferenceName {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalConstantName {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalSignalName {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalVariableName {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParenthesisName {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackagePathname {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AbsolutePathname {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelativePathname {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PathnameElement {}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoxName {}
