use super::{node_properties, FrontendGraphDataType, FrontendNodeType};
use crate::messages::layout::utility_types::layout_widget::LayoutGroup;

use graph_craft::concrete;
use graph_craft::document::*;
use graph_craft::document::{value::*, DocumentNodeImplementation};
use graph_craft::imaginate_input::ImaginateSamplingMethod;
use graph_craft::proto::{NodeIdentifier, Type};
use graphene_core::raster::Image;

use std::collections::VecDeque;

pub struct DocumentInputType {
	pub name: &'static str,
	pub data_type: FrontendGraphDataType,
	pub default: NodeInput,
}

impl DocumentInputType {
	pub const fn new(name: &'static str, tagged_value: TaggedValue, exposed: bool) -> Self {
		let data_type = FrontendGraphDataType::with_tagged_value(&tagged_value);
		let default = NodeInput::value(tagged_value, exposed);
		Self { name, data_type, default }
	}

	pub const fn _none() -> Self {
		Self {
			name: "None",
			data_type: FrontendGraphDataType::General,
			default: NodeInput::value(TaggedValue::None, false),
		}
	}
}

pub struct NodePropertiesContext<'a> {
	pub persistent_data: &'a crate::messages::portfolio::utility_types::PersistentData,
	pub document: &'a document_legacy::document::Document,
	pub responses: &'a mut VecDeque<crate::messages::prelude::Message>,
	pub layer_path: &'a [document_legacy::LayerId],
	pub nested_path: &'a [NodeId],
}

#[derive(Clone)]
pub enum NodeImplementation {
	ProtoNode(NodeIdentifier),
	DocumentNode(NodeNetwork),
}

impl NodeImplementation {
	pub const fn proto(name: &'static str, types: &'static [Type]) -> Self {
		Self::ProtoNode(NodeIdentifier::new(name, types))
	}
}

#[derive(Clone)]
pub struct DocumentNodeType {
	pub name: &'static str,
	pub category: &'static str,
	pub identifier: NodeImplementation,
	pub inputs: &'static [DocumentInputType],
	pub outputs: &'static [FrontendGraphDataType],
	pub properties: fn(&DocumentNode, NodeId, &mut NodePropertiesContext) -> Vec<LayoutGroup>,
}

fn document_node_types() -> Vec<DocumentNodeType> {
	let mut vec: Vec<_> = STATIC_NODES.to_vec();

	const INPUTS: &[DocumentInputType] = &[
		DocumentInputType::new("Image", TaggedValue::Image(Image::empty()), true),
		DocumentInputType::new("Radius", TaggedValue::U32(3), false),
		DocumentInputType::new("Sigma", TaggedValue::F64(1.), false),
	];

	let blur = DocumentNodeType {
		name: "Gaussian Blur",
		category: "Image Filters",
		identifier: NodeImplementation::DocumentNode(NodeNetwork {
			inputs: vec![0, 1, 1],
			output: 1,
			nodes: vec![
				(
					0,
					DocumentNode {
						name: "CacheNode".to_string(),
						inputs: vec![NodeInput::Network],
						implementation: DocumentNodeImplementation::Unresolved(NodeIdentifier::new("graphene_std::memo::CacheNode", &[concrete!("Image")])),
						metadata: Default::default(),
					},
				),
				(
					1,
					DocumentNode {
						name: "BlurNode".to_string(),
						inputs: vec![NodeInput::Node(0), NodeInput::Network, NodeInput::Network],
						implementation: DocumentNodeImplementation::Unresolved(NodeIdentifier::new("graphene_core::raster::BlurNode", &[])),
						metadata: Default::default(),
					},
				),
			]
			.into_iter()
			.collect(),
			..Default::default()
		}),
		inputs: INPUTS,
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::blur_image_properties,
	};
	vec.push(blur);
	vec
}

// We use the once cell for lazy initialization to avoid the overhead of reconstructing the node list every time.
// TODO: make document nodes not require a `'static` lifetime to avoid having to split the construction into const and non-const parts.
static DOCUMENT_NODE_TYPES: once_cell::sync::Lazy<Vec<DocumentNodeType>> = once_cell::sync::Lazy::new(document_node_types);

// TODO: Dynamic node library
static STATIC_NODES: &[DocumentNodeType] = &[
	DocumentNodeType {
		name: "Identity",
		category: "General",
		identifier: NodeImplementation::proto("graphene_core::ops::IdNode", &[concrete!("Any<'_>")]),
		inputs: &[DocumentInputType {
			name: "In",
			data_type: FrontendGraphDataType::General,
			default: NodeInput::Node(0),
		}],
		outputs: &[FrontendGraphDataType::General],
		properties: |_document_node, _node_id, _context| node_properties::string_properties("The identity node simply returns the input"),
	},
	DocumentNodeType {
		name: "Image",
		category: "Ignore",
		identifier: NodeImplementation::proto("graphene_core::ops::IdNode", &[concrete!("Any<'_>")]),
		inputs: &[DocumentInputType::new("Image", TaggedValue::Image(Image::empty()), false)],
		outputs: &[FrontendGraphDataType::Raster],
		properties: |_document_node, _node_id, _context| node_properties::string_properties("A bitmap image embedded in this node"),
	},
	DocumentNodeType {
		name: "Input",
		category: "Ignore",
		identifier: NodeImplementation::proto("graphene_core::ops::IdNode", &[concrete!("Any<'_>")]),
		inputs: &[DocumentInputType {
			name: "In",
			data_type: FrontendGraphDataType::Raster,
			default: NodeInput::Network,
		}],
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::input_properties,
	},
	DocumentNodeType {
		name: "Output",
		category: "Ignore",
		identifier: NodeImplementation::proto("graphene_core::ops::IdNode", &[concrete!("Any<'_>")]),
		inputs: &[DocumentInputType {
			name: "In",
			data_type: FrontendGraphDataType::Raster,
			default: NodeInput::value(TaggedValue::Image(Image::empty()), true),
		}],
		outputs: &[],
		properties: |_document_node, _node_id, _context| node_properties::string_properties("The graph's output is rendered into the frame".to_string()),
	},
	DocumentNodeType {
		name: "Grayscale",
		category: "Image Adjustments",
		identifier: NodeImplementation::proto("graphene_std::raster::GrayscaleNode", &[]),
		inputs: &[DocumentInputType::new("Image", TaggedValue::Image(Image::empty()), true)],
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::no_properties,
	},
	#[cfg(feature = "gpu")]
	DocumentNodeType {
		name: "GpuImage",
		category: "Image Adjustments",
		identifier: NodeImplementation::proto("graphene_std::executor::MapGpuSingleImageNode", &[concrete!("&TypeErasedNode")]),
		inputs: &[
			DocumentInputType::new("Image", TaggedValue::Image(Image::empty()), true),
			DocumentInputType {
				name: "Path",
				data_type: FrontendGraphDataType::Text,
				default: NodeInput::value(TaggedValue::String(String::new()), false),
			},
		],
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::gpu_map_properties,
	},
	#[cfg(feature = "quantization")]
	DocumentNodeType {
		name: "QuantizeImage",
		category: "Image Adjustments",
		identifier: NodeImplementation::proto("graphene_std::quantization::GenerateQuantizationNode", &[concrete!("&TypeErasedNode")]),
		inputs: &[
			DocumentInputType {
				name: "Image",
				data_type: FrontendGraphDataType::Raster,
				default: NodeInput::value(TaggedValue::Image(Image::empty()), true),
			},
			DocumentInputType {
				name: "samples",
				data_type: FrontendGraphDataType::Number,
				default: NodeInput::value(TaggedValue::U32(100), false),
			},
			DocumentInputType {
				name: "Fn index",
				data_type: FrontendGraphDataType::Number,
				default: NodeInput::value(TaggedValue::U32(0), false),
			},
		],
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::quantize_properties,
	},
	DocumentNodeType {
		name: "Cache",
		category: "Structural",
		identifier: NodeImplementation::proto("graphene_std::memo::CacheNode", &[concrete!("Image")]),
		inputs: &[DocumentInputType::new("Image", TaggedValue::Image(Image::empty()), true)],
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::no_properties,
	},
	DocumentNodeType {
		name: "Invert RGB",
		category: "Image Adjustments",
		identifier: NodeImplementation::proto("graphene_std::raster::InvertRGBNode", &[]),
		inputs: &[DocumentInputType::new("Image", TaggedValue::Image(Image::empty()), true)],
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::no_properties,
	},
	DocumentNodeType {
		name: "Hue/Saturation",
		category: "Image Adjustments",
		identifier: NodeImplementation::proto("graphene_std::raster::HueSaturationNode", &[concrete!("&TypeErasedNode")]),
		inputs: &[
			DocumentInputType::new("Image", TaggedValue::Image(Image::empty()), true),
			DocumentInputType::new("Hue Shift", TaggedValue::F64(0.), false),
			DocumentInputType::new("Saturation Shift", TaggedValue::F64(0.), false),
			DocumentInputType::new("Lightness Shift", TaggedValue::F64(0.), false),
		],
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::adjust_hsl_properties,
	},
	DocumentNodeType {
		name: "Brightness/Contrast",
		category: "Image Adjustments",
		identifier: NodeImplementation::proto("graphene_std::raster::BrightnessContrastNode", &[concrete!("&TypeErasedNode")]),
		inputs: &[
			DocumentInputType::new("Image", TaggedValue::Image(Image::empty()), true),
			DocumentInputType::new("Brightness", TaggedValue::F64(0.), false),
			DocumentInputType::new("Contrast", TaggedValue::F64(0.), false),
		],
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::brighten_image_properties,
	},
	DocumentNodeType {
		name: "Gamma",
		category: "Image Adjustments",
		identifier: NodeImplementation::proto("graphene_std::raster::GammaNode", &[concrete!("&TypeErasedNode")]),
		inputs: &[
			DocumentInputType::new("Image", TaggedValue::Image(Image::empty()), true),
			DocumentInputType::new("Gamma", TaggedValue::F64(1.), false),
		],
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::adjust_gamma_properties,
	},
	DocumentNodeType {
		name: "Opacity",
		category: "Image Adjustments",
		identifier: NodeImplementation::proto("graphene_std::raster::OpacityNode", &[concrete!("&TypeErasedNode")]),
		inputs: &[
			DocumentInputType::new("Image", TaggedValue::Image(Image::empty()), true),
			DocumentInputType::new("Factor", TaggedValue::F64(1.), false),
		],
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::multiply_opacity,
	},
	DocumentNodeType {
		name: "Posterize",
		category: "Image Adjustments",
		identifier: NodeImplementation::proto("graphene_std::raster::PosterizeNode", &[concrete!("&TypeErasedNode")]),
		inputs: &[
			DocumentInputType::new("Image", TaggedValue::Image(Image::empty()), true),
			DocumentInputType::new("Value", TaggedValue::F64(5.), false),
		],
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::posterize_properties,
	},
	DocumentNodeType {
		name: "Exposure",
		category: "Image Adjustments",
		identifier: NodeImplementation::proto("graphene_std::raster::ExposureNode", &[concrete!("&TypeErasedNode")]),
		inputs: &[
			DocumentInputType::new("Image", TaggedValue::Image(Image::empty()), true),
			DocumentInputType::new("Value", TaggedValue::F64(0.), false),
		],
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::exposure_properties,
	},
	IMAGINATE_NODE,
	DocumentNodeType {
		name: "Add",
		category: "Math",
		identifier: NodeImplementation::proto("graphene_core::ops::AddNode", &[concrete!("&TypeErasedNode")]),
		inputs: &[
			DocumentInputType::new("Input", TaggedValue::F64(0.), true),
			DocumentInputType::new("Addend", TaggedValue::F64(0.), true),
		],
		outputs: &[FrontendGraphDataType::Number],
		properties: node_properties::add_properties,
	},
	/*DocumentNodeType {
		name: "Unit Circle Generator",
		category: "Vector",
		identifier: NodeImplementation::proto("graphene_std::vector::generator_nodes::UnitCircleGenerator", &[]),
		inputs: &[DocumentInputType::none()],
		outputs: &[FrontendGraphDataType::Subpath],
		properties: node_properties::no_properties,
	},
	DocumentNodeType {
		name: "Unit Square Generator",
		category: "Vector",
		identifier: NodeImplementation::proto("graphene_std::vector::generator_nodes::UnitSquareGenerator", &[]),
		inputs: &[DocumentInputType::none()],
		outputs: &[FrontendGraphDataType::Subpath],
		properties: node_properties::no_properties,
	},
	DocumentNodeType {
		name: "Path Generator",
		category: "Vector",
		identifier: NodeImplementation::proto("graphene_core::ops::IdNode", &[concrete!("Any<'_>")]),
		inputs: &[DocumentInputType {
			name: "Path Data",
			data_type: FrontendGraphDataType::Subpath,
			default: NodeInput::value(TaggedValue::Subpath(Subpath::new()), false),
		}],
		outputs: &[FrontendGraphDataType::Subpath],
		properties: node_properties::no_properties,
	},
	DocumentNodeType {
		name: "Transform Subpath",
		category: "Vector",
		identifier: NodeImplementation::proto("graphene_std::vector::generator_nodes::TransformSubpathNode", &[]),
		inputs: &[
			DocumentInputType::new("Subpath", TaggedValue::Subpath(Subpath::empty()), true),
			DocumentInputType::new("Translation", TaggedValue::DVec2(DVec2::ZERO), false),
			DocumentInputType::new("Rotation", TaggedValue::F64(0.), false),
			DocumentInputType::new("Scale", TaggedValue::DVec2(DVec2::ONE), false),
			DocumentInputType::new("Skew", TaggedValue::DVec2(DVec2::ZERO), false),
		],
		outputs: &[FrontendGraphDataType::Subpath],
		properties: node_properties::transform_properties,
	},
	DocumentNodeType {
		name: "Blit Subpath",
		category: "Vector",
		identifier: NodeImplementation::proto("graphene_std::vector::generator_nodes::BlitSubpath", &[]),
		inputs: &[
			DocumentInputType::new("Image", TaggedValue::Image(Image::empty()), true),
			DocumentInputType::new("Subpath", TaggedValue::Subpath(Subpath::empty()), true),
		],
		outputs: &[FrontendGraphDataType::Raster],
		properties: node_properties::no_properties,
	},*/
];

pub const IMAGINATE_NODE: DocumentNodeType = DocumentNodeType {
	name: "Imaginate",
	category: "Image Synthesis",
	identifier: NodeImplementation::proto("graphene_std::raster::ImaginateNode", &[concrete!("&TypeErasedNode")]),
	inputs: &[
		DocumentInputType::new("Input Image", TaggedValue::Image(Image::empty()), true),
		DocumentInputType::new("Seed", TaggedValue::F64(0.), false),
		DocumentInputType::new("Resolution", TaggedValue::OptionalDVec2(None), false),
		DocumentInputType::new("Samples", TaggedValue::F64(30.), false),
		DocumentInputType::new("Sampling Method", TaggedValue::ImaginateSamplingMethod(ImaginateSamplingMethod::EulerA), false),
		DocumentInputType::new("Prompt Guidance", TaggedValue::F64(10.), false),
		DocumentInputType::new("Prompt", TaggedValue::String(String::new()), false),
		DocumentInputType::new("Negative Prompt", TaggedValue::String(String::new()), false),
		DocumentInputType::new("Adapt Input Image", TaggedValue::Bool(false), false),
		DocumentInputType::new("Image Creativity", TaggedValue::F64(66.), false),
		DocumentInputType::new("Masking Layer", TaggedValue::LayerPath(None), false),
		DocumentInputType::new("Inpaint", TaggedValue::Bool(true), false),
		DocumentInputType::new("Mask Blur", TaggedValue::F64(4.), false),
		DocumentInputType::new("Mask Starting Fill", TaggedValue::ImaginateMaskStartingFill(ImaginateMaskStartingFill::Fill), false),
		DocumentInputType::new("Improve Faces", TaggedValue::Bool(false), false),
		DocumentInputType::new("Tiling", TaggedValue::Bool(false), false),
		// Non-user status (is document input the right way to do this?)
		DocumentInputType::new("Cached Data", TaggedValue::RcImage(None), false),
		DocumentInputType::new("Percent Complete", TaggedValue::F64(0.), false),
		DocumentInputType::new("Status", TaggedValue::ImaginateStatus(ImaginateStatus::Idle), false),
	],
	outputs: &[FrontendGraphDataType::Raster],
	properties: node_properties::imaginate_properties,
};

pub fn resolve_document_node_type(name: &str) -> Option<&DocumentNodeType> {
	DOCUMENT_NODE_TYPES.iter().find(|node| node.name == name)
}

pub fn collect_node_types() -> Vec<FrontendNodeType> {
	DOCUMENT_NODE_TYPES
		.iter()
		.filter(|node_type| !node_type.category.eq_ignore_ascii_case("ignore"))
		.map(|node_type| FrontendNodeType::new(node_type.name, node_type.category))
		.collect()
}

impl DocumentNodeType {
	/// Generate a [`DocumentNodeImplementation`] from this node type, using a nested network.
	pub fn generate_implementation(&self) -> DocumentNodeImplementation {
		let num_inputs = self.inputs.len();

		let inner_network = match &self.identifier {
			NodeImplementation::ProtoNode(ident) => {
				NodeNetwork {
					inputs: (0..num_inputs).map(|_| 0).collect(),
					output: 0,
					nodes: [(
						0,
						DocumentNode {
							name: format!("{}_impl", self.name),
							// TODO: Allow inserting nodes that contain other nodes.
							implementation: DocumentNodeImplementation::Unresolved(ident.clone()),
							inputs: (0..num_inputs).map(|_| NodeInput::Network).collect(),
							metadata: DocumentNodeMetadata::default(),
						},
					)]
					.into_iter()
					.collect(),
					..Default::default()
				}
			}
			NodeImplementation::DocumentNode(network) => network.clone(),
		};
		DocumentNodeImplementation::Network(inner_network)
	}
}
