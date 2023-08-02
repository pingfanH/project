# [allow (non_snake_case)] # [allow (non_camel_case_types)] # [allow (clippy :: style)] # [allow (clippy :: complexity)] # [allow (unused_braces)] # [allow (clippy :: erasing_op)] # [allow (clippy :: approx_constant)] # [allow (clippy :: eq_op)] # [allow (clippy :: cmp_owned)] # [allow (clippy :: redundant_clone)] mod slint_generatedSetting {
     use slint :: private_unstable_api :: re_exports :: * ;
     use slint :: private_unstable_api :: re_exports as sp ;
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerColorSchemeSelector_4 {
         r#dark_color_scheme : sp :: Property < bool > , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerSetting >> , }
     impl InnerColorSchemeSelector_4 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ComponentVTable , InnerSetting >) {
             # ! [allow (unused)] self . root . set (VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerColorSchemeSelector_4 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ()) . window ()) . dark_color_scheme ()) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerPalette_5 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerSetting >> , }
     impl InnerPalette_5 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ComponentVTable , InnerSetting >) {
             # ! [allow (unused)] self . root . set (VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerStyleMetrics_6 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerSetting >> , }
     impl InnerStyleMetrics_6 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ComponentVTable , InnerSetting >) {
             # ! [allow (unused)] self . root . set (VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerSetting {
         r#root_window_1 : sp :: r#WindowItem , r#root_2 : sp :: r#Empty , r#rectangle_3 : sp :: r#Rectangle , r#root_window_1_layoutinfo_h : sp :: Property < r#LayoutInfo > , r#root_window_1_layoutinfo_v : sp :: Property < r#LayoutInfo > , r#root_window_1_showsetting : sp :: Callback < () , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ComponentVTable , InnerSetting >> , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerSetting >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , globals : Globals_Setting , window_adapter_ : sp :: OnceCell < sp :: Rc < dyn sp :: WindowAdapter >> , }
     impl InnerSetting {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self > , root : & sp :: VRc < sp :: ComponentVTable , InnerSetting > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             Property :: link_two_way (({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_window_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , ({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_2 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self)) ;
             Property :: link_two_way (({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_window_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , ({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_2 }
             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self)) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSetting :: FIELD_OFFSETS . r#root_window_1 }
                 + r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_4 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_4 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4280032284f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294638330f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSetting :: FIELD_OFFSETS . r#root_window_1_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerSetting :: FIELD_OFFSETS . r#root_window_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ()))) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 1000f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSetting :: FIELD_OFFSETS . r#root_window_1_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerSetting :: FIELD_OFFSETS . r#root_window_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter ()))) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 670f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSetting :: FIELD_OFFSETS . r#root_2 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSetting :: FIELD_OFFSETS . r#root_2 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSetting :: FIELD_OFFSETS . r#root_2 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((0f64 as f64) - (0f64 as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSetting :: FIELD_OFFSETS . r#root_2 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((0f64 as f64) - (0f64 as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerSetting :: FIELD_OFFSETS . r#rectangle_3 }
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
                     * & InnerSetting :: FIELD_OFFSETS . r#rectangle_3 }
                 + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSetting :: FIELD_OFFSETS . r#rectangle_3 }
                 + r#Rectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_window_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_window_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_window_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_window_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_window_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_window_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSetting :: FIELD_OFFSETS . r#rectangle_3 }
             + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSetting :: FIELD_OFFSETS . r#rectangle_3 }
             + r#Rectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSetting :: FIELD_OFFSETS . r#rectangle_3 }
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
                     * & InnerSetting :: FIELD_OFFSETS . r#root_window_1_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerSetting :: FIELD_OFFSETS . r#root_window_1_layoutinfo_v }
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
     impl InnerSetting {
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
             3usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 3u32 , parent_index : 1u32 , item_array_index : 2u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [vtable :: VOffset < InnerSetting , ItemVTable , vtable :: AllowPin > ;
             3usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_window_1 }
            ) , VOffset :: new ({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_2 }
            ) , VOffset :: new ({
                 * & InnerSetting :: FIELD_OFFSETS . r#rectangle_3 }
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
     impl sp :: PinnedDrop for InnerSetting {
         fn drop (self : core :: pin :: Pin < & mut InnerSetting >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerSetting) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter () {
                 sp :: unregister_component (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: Component for InnerSetting {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerSetting > , order : sp :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
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
     pub struct r#Setting (vtable :: VRc < sp :: ComponentVTable , InnerSetting >) ;
     impl r#Setting {
         pub fn new () -> core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerSetting :: new () ? ;
             inner . globals . global_ColorSchemeSelector_4 . clone () . init (& inner) ;
             inner . globals . global_Palette_5 . clone () . init (& inner) ;
             inner . globals . global_StyleMetrics_6 . clone () . init (& inner) ;
             InnerSetting :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn invoke_showsetting (& self ,) -> () {
             let _self = vtable :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_window_1_showsetting }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_showsetting (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = vtable :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerSetting :: FIELD_OFFSETS . r#root_window_1_showsetting }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         }
     impl From < r#Setting > for vtable :: VRc < sp :: ComponentVTable , InnerSetting > {
         fn from (value : r#Setting) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#Setting {
         type Inner = InnerSetting ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (& self . 0) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn from_inner (inner : vtable :: VRc < sp :: ComponentVTable , InnerSetting >) -> Self {
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
     # [allow (dead_code)] struct Globals_Setting {
         global_ColorSchemeSelector_4 : :: core :: pin :: Pin < sp :: Rc < InnerColorSchemeSelector_4 >> , global_Palette_5 : :: core :: pin :: Pin < sp :: Rc < InnerPalette_5 >> , global_StyleMetrics_6 : :: core :: pin :: Pin < sp :: Rc < InnerStyleMetrics_6 >> , }
     impl Default for Globals_Setting {
         fn default () -> Self {
             Self {
                 global_ColorSchemeSelector_4 : InnerColorSchemeSelector_4 :: new () , global_Palette_5 : InnerPalette_5 :: new () , global_StyleMetrics_6 : InnerStyleMetrics_6 :: new () , }
             }
         }
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_1_1 = slint :: VersionCheck_1_1_1 ;
     }
 pub use slint_generatedSetting :: {
     r#Setting }
 ;
 pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
