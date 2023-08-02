# [allow (non_snake_case)] # [allow (non_camel_case_types)] # [allow (clippy :: style)] # [allow (clippy :: complexity)] # [allow (unused_braces)] # [allow (clippy :: erasing_op)] # [allow (clippy :: approx_constant)] # [allow (clippy :: eq_op)] # [allow (clippy :: cmp_owned)] # [allow (clippy :: redundant_clone)] mod slint_generatedApp {
     use slint :: private_unstable_api :: re_exports :: * ;
     use slint :: private_unstable_api :: re_exports as sp ;
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerColorSchemeSelector_21 {
         r#dark_color_scheme : sp :: Property < bool > , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerApp >> , }
     impl InnerColorSchemeSelector_21 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ComponentVTable , InnerApp >) {
             # ! [allow (unused)] self . root . set (VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerColorSchemeSelector_21 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ()) . window ()) . dark_color_scheme ()) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerPalette_22 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerApp >> , }
     impl InnerPalette_22 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ComponentVTable , InnerApp >) {
             # ! [allow (unused)] self . root . set (VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerStyleMetrics_23 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerApp >> , }
     impl InnerStyleMetrics_23 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ComponentVTable , InnerApp >) {
             # ! [allow (unused)] self . root . set (VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct Innersearch_root_13 {
         r#root_13 : sp :: r#Empty , r#area_14 : sp :: r#TouchArea , r#rectangle_15 : sp :: r#Rectangle , r#image_16 : sp :: r#ImageItem , r#textinput_17 : sp :: r#TextInput , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ComponentVTable , Innersearch_root_13 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl Innersearch_root_13 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self > , root : & sp :: VRc < sp :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#area_14 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#area_14 }
                 + r#TouchArea :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#area_14 }
                 + r#TouchArea :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#rectangle_15 }
             + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (2165060178f64 as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#rectangle_15 }
                 + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#rectangle_15 }
                 + r#Rectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#image_16 }
                 + r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & Innersearch_root_13 :: FIELD_OFFSETS . r#area_14 }
                     + r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (2217290025f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = r#PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 250f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#image_16 }
                 + r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (15f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#image_16 }
             + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#image_16 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#image_16 }
             + r#ImageItem :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#image_16 }
             + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
                 + r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_21 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_21 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("方正少儿简体")) as sp :: SharedString }
            ) ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
                 + r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (4286611584f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
                 + r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
                 + r#TextInput :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) / (5f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
                 + r#TextInput :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) / (6f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#area_14 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#area_14 }
             + r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#area_14 }
             + r#TouchArea :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#area_14 }
             + r#TouchArea :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#rectangle_15 }
             + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#rectangle_15 }
             + r#Rectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#rectangle_15 }
             + r#Rectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#image_16 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#image_16 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#image_16 }
             + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#image_16 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#image_16 }
             + r#ImageItem :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#image_16 }
             + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#input_type) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
             + r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct Innerplayer_root_18 {
         r#root_18 : sp :: r#Empty , r#rectangle_19 : sp :: r#Rectangle , r#image_20 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ComponentVTable , Innerplayer_root_18 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl Innerplayer_root_18 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self > , root : & sp :: VRc < sp :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#rectangle_19 }
             + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (2165060178f64 as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerplayer_root_18 :: FIELD_OFFSETS . r#rectangle_19 }
                 + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & Innerplayer_root_18 :: FIELD_OFFSETS . r#root_18 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerplayer_root_18 :: FIELD_OFFSETS . r#rectangle_19 }
                 + r#Rectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & Innerplayer_root_18 :: FIELD_OFFSETS . r#root_18 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#image_20 }
             + r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967295f64 as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerplayer_root_18 :: FIELD_OFFSETS . r#image_20 }
                 + r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (25f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#image_20 }
             + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#image_20 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#image_20 }
             + r#ImageItem :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerplayer_root_18 :: FIELD_OFFSETS . r#image_20 }
                 + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & Innerplayer_root_18 :: FIELD_OFFSETS . r#root_18 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) / (2.8f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#rectangle_19 }
             + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#rectangle_19 }
             + r#Rectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#rectangle_19 }
             + r#Rectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#image_20 }
             + r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#image_20 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#image_20 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#image_20 }
             + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#image_20 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#image_20 }
             + r#ImageItem :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & Innerplayer_root_18 :: FIELD_OFFSETS . r#root_18 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & Innerplayer_root_18 :: FIELD_OFFSETS . r#root_18 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerApp {
         r#root_1 : sp :: r#WindowItem , r#rectangle_2 : sp :: r#Rectangle , r#image_3 : sp :: r#ImageItem , r#CloseArea_4 : sp :: r#TouchArea , r#image_5 : sp :: r#ImageItem , r#SettingArea_6 : sp :: r#TouchArea , r#image_7 : sp :: r#ImageItem , r#UserArea_8 : sp :: r#TouchArea , r#search_9 : Innersearch_root_13 , r#player_10 : Innerplayer_root_18 , r#root_1_showsetting : sp :: Callback < () , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ComponentVTable , InnerApp >> , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , globals : Globals_App , window_adapter_ : sp :: OnceCell < sp :: Rc < dyn sp :: WindowAdapter >> , }
     impl InnerApp {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self > , root : & sp :: VRc < sp :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             Innersearch_root_13 :: init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#search_9 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 5u32 - 1 , tree_index_of_first_child + 10u32 - 1) ;
             Innerplayer_root_18 :: init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#player_10 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 6u32 - 1 , tree_index_of_first_child + 14u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                 + r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_21 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_21 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4280032284f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294638330f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Slint Window")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_2 }
             + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (125f64 as _ , [sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4279438375f64 as u32) , position : 0.9f64 as _ }
                 , sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4279176214f64 as u32) , position : 1f64 as _ }
                 , sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4280029754f64 as u32) , position : 1f64 as _ }
                 , sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4280226881f64 as u32) , position : 1f64 as _ }
                 , sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4279764038f64 as u32) , position : 1f64 as _ }
                ]))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#rectangle_2 }
                 + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#rectangle_2 }
                 + r#Rectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_3 }
             + r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4292860891f64 as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#image_3 }
                 + r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (25f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_3 }
             + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_3 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#image_3 }
                 + r#ImageItem :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (60f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_3 }
             + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#CloseArea_4 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#CloseArea_4 }
                 + r#TouchArea :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (25f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#CloseArea_4 }
             + r#TouchArea :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_5 }
             + r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4292860891f64 as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#image_5 }
                 + r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (25f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_5 }
             + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_5 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#image_5 }
                 + r#ImageItem :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (120f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_5 }
             + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (22f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerApp :: FIELD_OFFSETS . r#SettingArea_6 }
                 + r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ()) . window ()) . show_popup (& VRc :: into_dyn ({
                                 let instance = InnerComponent_popup_11 :: new (_self . self_weak . get () . unwrap () . clone ()) ;
                                 InnerComponent_popup_11 :: user_init (sp :: VRc :: map (instance . clone () , | x | x)) ;
                                 instance . into () }
                            ) , Point :: new (20f64 as sp :: Coord , 20f64 as sp :: Coord) , true , & ItemRc :: new (VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () as usize + 3usize - 1)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#SettingArea_6 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#SettingArea_6 }
                 + r#TouchArea :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (25f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#SettingArea_6 }
             + r#TouchArea :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_7 }
             + r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4292860891f64 as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#image_7 }
                 + r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (25f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_7 }
             + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_7 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#image_7 }
                 + r#ImageItem :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (180f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_7 }
             + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (22f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#UserArea_8 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#UserArea_8 }
                 + r#TouchArea :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (25f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#UserArea_8 }
             + r#TouchArea :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#search_9 }
             + {
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (44f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerApp :: FIELD_OFFSETS . r#search_9 }
                 + {
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) / (5f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerApp :: FIELD_OFFSETS . r#search_9 }
                 + {
                     * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) / (5f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerApp :: FIELD_OFFSETS . r#search_9 }
             + {
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (17f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#player_10 }
             + {
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#root_18 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (90f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerApp :: FIELD_OFFSETS . r#player_10 }
                 + {
                     * & Innerplayer_root_18 :: FIELD_OFFSETS . r#root_18 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (15f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerApp :: FIELD_OFFSETS . r#player_10 }
                 + {
                     * & Innerplayer_root_18 :: FIELD_OFFSETS . r#root_18 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (15f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerApp :: FIELD_OFFSETS . r#player_10 }
                 + {
                     * & Innerplayer_root_18 :: FIELD_OFFSETS . r#root_18 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (100f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_2 }
             + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_2 }
             + r#Rectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_2 }
             + r#Rectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_3 }
             + r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_3 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_3 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_3 }
             + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_3 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_3 }
             + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#CloseArea_4 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#CloseArea_4 }
             + r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#CloseArea_4 }
             + r#TouchArea :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#CloseArea_4 }
             + r#TouchArea :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#CloseArea_4 }
             + r#TouchArea :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_5 }
             + r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_5 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_5 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_5 }
             + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_5 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_5 }
             + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#SettingArea_6 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#SettingArea_6 }
             + r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#SettingArea_6 }
             + r#TouchArea :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#SettingArea_6 }
             + r#TouchArea :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#SettingArea_6 }
             + r#TouchArea :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_7 }
             + r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_7 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_7 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_7 }
             + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_7 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_7 }
             + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#UserArea_8 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#UserArea_8 }
             + r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#UserArea_8 }
             + r#TouchArea :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#UserArea_8 }
             + r#TouchArea :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#UserArea_8 }
             + r#TouchArea :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#search_9 }
             + {
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#search_9 }
             + {
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 InnerApp :: FIELD_OFFSETS . r#player_10 }
             + {
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#root_18 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             Innersearch_root_13 :: user_init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#search_9 }
             . apply_pin (x)) ,) ;
             Innerplayer_root_18 :: user_init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#player_10 }
             . apply_pin (x)) ,) ;
             (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ()) . renderer () . register_font_from_memory (SLINT_EMBEDDED_RESOURCE_4 . into ()) . unwrap () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ())) ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = 1000f64 as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ())) ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = 670f64 as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 5usize => {
                     * & Self :: FIELD_OFFSETS . r#search_9 }
                 . apply_pin (_self) . accessible_role (0) , 10usize ..= 13usize => {
                     * & Self :: FIELD_OFFSETS . r#search_9 }
                 . apply_pin (_self) . accessible_role (index - 10usize + 1) , 6usize => {
                     * & Self :: FIELD_OFFSETS . r#player_10 }
                 . apply_pin (_self) . accessible_role (0) , 14usize ..= 15usize => {
                     * & Self :: FIELD_OFFSETS . r#player_10 }
                 . apply_pin (_self) . accessible_role (index - 14usize + 1) , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (5usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#search_9 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (10usize ..= 13usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#search_9 }
                 . apply_pin (_self) . accessible_string_property (index - 10usize + 1 , what) , (6usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#player_10 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (14usize ..= 15usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#player_10 }
                 . apply_pin (_self) . accessible_string_property (index - 14usize + 1 , what) , _ => Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_popup_11 {
         r#popup_11 : sp :: r#WindowItem , r#rectangle_12 : sp :: r#Rectangle , r#popup_11_layoutinfo_h : sp :: Property < r#LayoutInfo > , r#popup_11_layoutinfo_v : sp :: Property < r#LayoutInfo > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ComponentVTable , InnerComponent_popup_11 >> , parent : sp :: VWeakMapped :: < sp :: ComponentVTable , InnerApp > , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_popup_11 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self > , root : & sp :: VRc < sp :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
             + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (50f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ()))) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ()))) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
             + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (50f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#rectangle_12 }
             + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967040f64 as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#rectangle_12 }
                 + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
                     + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#rectangle_12 }
                 + r#Rectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
             + r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
             + r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
             + r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
             + r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
             + r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
             + r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
             + r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#rectangle_12 }
             + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#rectangle_12 }
             + r#Rectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#rectangle_12 }
             + r#Rectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => Default :: default () , }
             }
         }
     impl InnerComponent_popup_11 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ComponentVTable , InnerApp >) -> vtable :: VRc < sp :: ComponentVTable , Self > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ComponentVTable , InnerApp > ;
             let self_rc = VRc :: new (_self) ;
             let _self = self_rc . as_pin_ref () ;
             sp :: register_component (_self , Self :: item_array () , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             Self :: user_init (sp :: VRc :: map (self_rc . clone () , | x | x)) ;
             self_rc }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [vtable :: VOffset < InnerComponent_popup_11 , ItemVTable , vtable :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#popup_11 }
            ) , VOffset :: new ({
                 * & InnerComponent_popup_11 :: FIELD_OFFSETS . r#rectangle_12 }
            )])) }
         # [allow (unused)] fn window_adapter (& self) -> Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter () }
         # [allow (unused)] fn maybe_window_adapter (& self) -> Option < Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter ()) }
         }
     impl sp :: PinnedDrop for InnerComponent_popup_11 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_popup_11 >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerComponent_popup_11) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter () {
                 sp :: unregister_component (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: Component for InnerComponent_popup_11 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_popup_11 > , order : sp :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         }
     impl InnerApp {
         pub fn new () -> core :: result :: Result < vtable :: VRc < sp :: ComponentVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ;
             let mut _self = Self :: default () ;
             let self_rc = VRc :: new (_self) ;
             let _self = self_rc . as_pin_ref () ;
             sp :: register_component (_self , Self :: item_array () , (* & self_rc) . maybe_window_adapter ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & self_rc , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             16usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 6u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 7u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 8u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 9u32 , parent_index : 0u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 10u32 , parent_index : 0u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 14u32 , parent_index : 0u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 8u32 , parent_index : 2u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 9u32 , parent_index : 3u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 10u32 , parent_index : 4u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 5u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 5u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 5u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 5u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 16u32 , parent_index : 6u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 16u32 , parent_index : 6u32 , item_array_index : 15u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [vtable :: VOffset < InnerApp , ItemVTable , vtable :: AllowPin > ;
             16usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_2 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_3 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_5 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#image_7 }
            ) , VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#search_9 }
             + {
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#root_13 }
            ) , VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#player_10 }
             + {
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#root_18 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#CloseArea_4 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#SettingArea_6 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#UserArea_8 }
            ) , VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#search_9 }
             + {
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#area_14 }
            ) , VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#search_9 }
             + {
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#rectangle_15 }
            ) , VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#search_9 }
             + {
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#image_16 }
            ) , VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#search_9 }
             + {
                 * & Innersearch_root_13 :: FIELD_OFFSETS . r#textinput_17 }
            ) , VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#player_10 }
             + {
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#rectangle_19 }
            ) , VOffset :: new ({
                 InnerApp :: FIELD_OFFSETS . r#player_10 }
             + {
                 * & Innerplayer_root_18 :: FIELD_OFFSETS . r#image_20 }
            )])) }
         # [allow (unused)] fn window_adapter (& self) -> Rc < dyn sp :: WindowAdapter > {
             self . window_adapter_ref () . unwrap () . clone () }
         fn window_adapter_ref (& self ,) -> Result < & Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter_ . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let self_rc = VRcMapped :: origin (& self . self_weak . get () . unwrap () . upgrade () . unwrap ()) ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& self_rc) ;
                 core :: result :: Result :: Ok (adapter) }
            ) }
         # [allow (unused)] fn maybe_window_adapter (& self) -> Option < Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter_ . get () . cloned () }
         }
     impl sp :: PinnedDrop for InnerApp {
         fn drop (self : core :: pin :: Pin < & mut InnerApp >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerApp) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter () {
                 sp :: unregister_component (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: Component for InnerApp {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerApp > , order : sp :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         }
     pub struct r#App (vtable :: VRc < sp :: ComponentVTable , InnerApp >) ;
     impl r#App {
         pub fn new () -> core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerApp :: new () ? ;
             inner . globals . global_ColorSchemeSelector_21 . clone () . init (& inner) ;
             inner . globals . global_Palette_22 . clone () . init (& inner) ;
             inner . globals . global_StyleMetrics_23 . clone () . init (& inner) ;
             InnerApp :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn invoke_showsetting (& self ,) -> () {
             let _self = vtable :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1_showsetting }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_showsetting (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = vtable :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1_showsetting }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         }
     impl From < r#App > for vtable :: VRc < sp :: ComponentVTable , InnerApp > {
         fn from (value : r#App) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#App {
         type Inner = InnerApp ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (& self . 0) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn from_inner (inner : vtable :: VRc < sp :: ComponentVTable , InnerApp >) -> Self {
             Self (inner) }
         fn run (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             core :: result :: Result :: Ok (()) }
         fn show (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     # [allow (dead_code)] struct Globals_App {
         global_ColorSchemeSelector_21 : :: core :: pin :: Pin < sp :: Rc < InnerColorSchemeSelector_21 >> , global_Palette_22 : :: core :: pin :: Pin < sp :: Rc < InnerPalette_22 >> , global_StyleMetrics_23 : :: core :: pin :: Pin < sp :: Rc < InnerStyleMetrics_23 >> , }
     impl Default for Globals_App {
         fn default () -> Self {
             Self {
                 global_ColorSchemeSelector_21 : InnerColorSchemeSelector_21 :: new () , global_Palette_22 : InnerPalette_22 :: new () , global_StyleMetrics_23 : InnerStyleMetrics_23 :: new () , }
             }
         }
     static SLINT_EMBEDDED_RESOURCE_2 : & 'static [u8] = :: core :: include_bytes ! ("D:\\project\\RUST\\project\\src\\assets\\icon\\xmark.svg") ;
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = :: core :: include_bytes ! ("D:\\project\\RUST\\project\\src\\assets\\icon\\search.svg") ;
     static SLINT_EMBEDDED_RESOURCE_4 : & 'static [u8] = :: core :: include_bytes ! ("D:\\project\\RUST\\project\\src\\assets\\fonts\\fangzheng.ttf") ;
     static SLINT_EMBEDDED_RESOURCE_1 : & 'static [u8] = :: core :: include_bytes ! ("D:\\project\\RUST\\project\\src\\assets\\icon\\setting.svg") ;
     static SLINT_EMBEDDED_RESOURCE_3 : & 'static [u8] = :: core :: include_bytes ! ("D:\\project\\RUST\\project\\src\\assets\\icon\\user.svg") ;
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_1_1 = slint :: VersionCheck_1_1_1 ;
     }
 pub use slint_generatedApp :: {
     r#App }
 ;
 pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
