declare class Vector2
    x: number
    y: number
end
declare class Vector3 extends Vector2
    z: number
end
declare class Vector4 extends Vector3
    w: number
end
declare function vector(x: number, y: number, z: number, w: number): Vector4
export type Color = Vector4

export type ColorTable = {
    ALICE_BLUE: Color,
    ANTIQUE_WHITE: Color,
    AQUA: Color,
    AQUAMARINE: Color,
    AZURE: Color,
    BEIGE: Color,
    BISQUE: Color,
    BLACK: Color,
    BLANCHED_ALMOND: Color,
    BLUE: Color,
    BLUE_VIOLET: Color,
    BROWN: Color,
    BURLY_WOOD: Color,
    CADET_BLUE: Color,
    CHARTREUSE: Color,
    CHOCOLATE: Color,
    CORAL: Color,
    CORNFLOWER_BLUE: Color,
    CORNSILK: Color,
    CRIMSON: Color,
    CYAN: Color,
    DARK_BLUE: Color,
    DARK_CYAN: Color,
    DARK_GOLDEN_ROD: Color,
    DARK_GRAY: Color,
    DARK_GREEN: Color,
    DARK_KHAKI: Color,
    DARK_MAGENTA: Color,
    DARK_OLIVE_GREEN: Color,
    DARKORANGE: Color,
    DARK_ORCHID: Color,
    DARK_RED: Color,
    DARK_SALMON: Color,
    DARK_SEA_GREEN: Color,
    DARK_SLATE_BLUE: Color,
    DARK_SLATE_GRAY: Color,
    DARK_TURQUOISE: Color,
    DARK_VIOLET: Color,
    DEEP_PINK: Color,
    DEEP_SKY_BLUE: Color,
    DIM_GRAY: Color,
    DODGER_BLUE: Color,
    FIRE_BRICK: Color,
    FLORAL_WHITE: Color,
    FOREST_GREEN: Color,
    FUCHSIA: Color,
    GAINSBORO: Color,
    GHOST_WHITE: Color,
    GOLD: Color,
    GOLDEN_ROD: Color,
    GRAY: Color,
    GREEN: Color,
    GREEN_YELLOW: Color,
    HONEY_DEW: Color,
    HOT_PINK: Color,
    INDIAN_RED: Color,
    INDIGO: Color,
    IVORY: Color,
    KHAKI: Color,
    LAVENDER: Color,
    LAVENDER_BLUSH: Color,
    LAWN_GREEN: Color,
    LEMON_CHIFFON: Color,
    LIGHT_BLUE: Color,
    LIGHT_CORAL: Color,
    LIGHT_CYAN: Color,
    LIGHT_GOLDEN_ROD_YELLOW: Color,
    LIGHT_GREY: Color,
    LIGHT_GREEN: Color,
    LIGHT_PINK: Color,
    LIGHT_SALMON: Color,
    LIGHT_SEA_GREEN: Color,
    LIGHT_SKY_BLUE: Color,
    LIGHT_SLATE_GRAY: Color,
    LIGHT_STEEL_BLUE: Color,
    LIGHT_YELLOW: Color,
    LIME: Color,
    LIME_GREEN: Color,
    LINEN: Color,
    MAGENTA: Color,
    MAROON: Color,
    MEDIUM_AQUA_MARINE: Color,
    MEDIUM_BLUE: Color,
    MEDIUM_ORCHID: Color,
    MEDIUM_PURPLE: Color,
    MEDIUM_SEA_GREEN: Color,
    MEDIUM_SLATE_BLUE: Color,
    MEDIUM_SPRING_GREEN: Color,
    MEDIUM_TURQUOISE: Color,
    MEDIUM_VIOLET_RED: Color,
    MIDNIGHT_BLUE: Color,
    MINT_CREAM: Color,
    MISTY_ROSE: Color,
    MOCCASIN: Color,
    NAVAJO_WHITE: Color,
    NAVY: Color,
    OLD_LACE: Color,
    OLIVE: Color,
    OLIVE_DRAB: Color,
    ORANGE: Color,
    ORANGE_RED: Color,
    ORCHID: Color,
    PALE_GOLDEN_ROD: Color,
    PALE_GREEN: Color,
    PALE_TURQUOISE: Color,
    PALE_VIOLET_RED: Color,
    PAPAYA_WHIP: Color,
    PEACH_PUFF: Color,
    PERU: Color,
    PINK: Color,
    PLUM: Color,
    POWDER_BLUE: Color,
    PURPLE: Color,
    RED: Color,
    ROSY_BROWN: Color,
    ROYAL_BLUE: Color,
    SADDLE_BROWN: Color,
    SALMON: Color,
    SANDY_BROWN: Color,
    SEA_GREEN: Color,
    SEA_SHELL: Color,
    SIENNA: Color,
    SILVER: Color,
    SKY_BLUE: Color,
    SLATE_BLUE: Color,
    SLATE_GRAY: Color,
    SNOW: Color,
    SPRING_GREEN: Color,
    STEEL_BLUE: Color,
    TAN: Color,
    TEAL: Color,
    THISTLE: Color,
    TOMATO: Color,
    TURQUOISE: Color,
    VIOLET: Color,
    WHEAT: Color,
    WHITE: Color,
    WHITE_SMOKE: Color,
    YELLOW: Color,
    YELLOW_GREEN: Color,
}
declare class CkiaEnumItem
end
declare class CkiaEnumClass
end
declare class affinity extends CkiaEnumItem
end
declare class affinity_internal extends CkiaEnumClass
    downstream : affinity
    upstream : affinity
end

declare class alphatype extends CkiaEnumItem
end
declare class alphatype_internal extends CkiaEnumClass
    opaque : alphatype
    premul : alphatype
    unknown : alphatype
    unpremul : alphatype
end

declare class bitmapAllocflags extends CkiaEnumItem
end
declare class bitmapAllocflags_internal extends CkiaEnumClass
    none : bitmapAllocflags
    zeroPixels : bitmapAllocflags
end

declare class blendmode extends CkiaEnumItem
end
declare class blendmode_internal extends CkiaEnumClass
    clear : blendmode
    color : blendmode
    colorburn : blendmode
    colordodge : blendmode
    darken : blendmode
    difference : blendmode
    dst : blendmode
    dstatop : blendmode
    dstin : blendmode
    dstout : blendmode
    dstover : blendmode
    exclusion : blendmode
    hardlight : blendmode
    hue : blendmode
    lighten : blendmode
    luminosity : blendmode
    modulate : blendmode
    multiply : blendmode
    overlay : blendmode
    plus : blendmode
    saturation : blendmode
    screen : blendmode
    softlight : blendmode
    src : blendmode
    srcatop : blendmode
    srcin : blendmode
    srcout : blendmode
    srcover : blendmode
    xor : blendmode
end

declare class blurstyle extends CkiaEnumItem
end
declare class blurstyle_internal extends CkiaEnumClass
    inner : blurstyle
    normal : blurstyle
    outer : blurstyle
    solid : blurstyle
end

declare class clipop extends CkiaEnumItem
end
declare class clipop_internal extends CkiaEnumClass
    difference : clipop
    intersect : clipop
end

declare class codecResult extends CkiaEnumItem
end
declare class codecResult_internal extends CkiaEnumClass
    couldNotRewind : codecResult
    errorInInput : codecResult
    incompleteInput : codecResult
    internalError : codecResult
    invalidConversion : codecResult
    invalidInput : codecResult
    invalidParameters : codecResult
    invalidScale : codecResult
    success : codecResult
    unimplemented : codecResult
end

declare class codecScanlineOrder extends CkiaEnumItem
end
declare class codecScanlineOrder_internal extends CkiaEnumClass
    bottomUp : codecScanlineOrder
    topDown : codecScanlineOrder
end

declare class codecZeroInitialized extends CkiaEnumItem
end
declare class codecZeroInitialized_internal extends CkiaEnumClass
    no : codecZeroInitialized
    yes : codecZeroInitialized
end

declare class codecanimationBlend extends CkiaEnumItem
end
declare class codecanimationBlend_internal extends CkiaEnumClass
    src : codecanimationBlend
    srcOver : codecanimationBlend
end

declare class codecanimationDisposalmethod extends CkiaEnumItem
end
declare class codecanimationDisposalmethod_internal extends CkiaEnumClass
    keep : codecanimationDisposalmethod
    restoreBgColor : codecanimationDisposalmethod
    restorePrevious : codecanimationDisposalmethod
end

declare class colorChannel extends CkiaEnumItem
end
declare class colorChannel_internal extends CkiaEnumClass
    a : colorChannel
    b : colorChannel
    g : colorChannel
    r : colorChannel
end

declare class colortype extends CkiaEnumItem
end
declare class colortype_internal extends CkiaEnumClass
    a16Float : colortype
    a16Unorm : colortype
    alpha8 : colortype
    argb4444 : colortype
    bgr101010x : colortype
    bgr101010xXr : colortype
    bgra1010102 : colortype
    bgra8888 : colortype
    gray8 : colortype
    r16g16Float : colortype
    r16g16Unorm : colortype
    r16g16b16a16Unorm : colortype
    r8Unorm : colortype
    r8g8Unorm : colortype
    rgb101010x : colortype
    rgb565 : colortype
    rgb888x : colortype
    rgba1010102 : colortype
    rgba10x6 : colortype
    rgba8888 : colortype
    rgbaF16 : colortype
    rgbaF16Norm : colortype
    rgbaF32 : colortype
    srgba8888 : colortype
    unknown : colortype
end

declare class encodedImageFormat extends CkiaEnumItem
end
declare class encodedImageFormat_internal extends CkiaEnumClass
    astc : encodedImageFormat
    avif : encodedImageFormat
    bmp : encodedImageFormat
    dng : encodedImageFormat
    gif : encodedImageFormat
    heif : encodedImageFormat
    ico : encodedImageFormat
    jpeg : encodedImageFormat
    jpegxl : encodedImageFormat
    ktx : encodedImageFormat
    pkm : encodedImageFormat
    png : encodedImageFormat
    wbmp : encodedImageFormat
    webp : encodedImageFormat
end

declare class encodedorigin extends CkiaEnumItem
end
declare class encodedorigin_internal extends CkiaEnumClass
    bottomLeft : encodedorigin
    bottomRight : encodedorigin
    leftBottom : encodedorigin
    leftTop : encodedorigin
    rightBottom : encodedorigin
    rightTop : encodedorigin
    topLeft : encodedorigin
    topRight : encodedorigin
end

declare class filterMode extends CkiaEnumItem
end
declare class filterMode_internal extends CkiaEnumClass
    linear : filterMode
    nearest : filterMode
end

declare class fontEdging extends CkiaEnumItem
end
declare class fontEdging_internal extends CkiaEnumClass
    alias : fontEdging
    antialias : fontEdging
    subpixelAntialias : fontEdging
end

declare class fontHinting extends CkiaEnumItem
end
declare class fontHinting_internal extends CkiaEnumClass
    full : fontHinting
    none : fontHinting
    normal : fontHinting
    slight : fontHinting
end

declare class fontStyleSlant extends CkiaEnumItem
end
declare class fontStyleSlant_internal extends CkiaEnumClass
    italic : fontStyleSlant
    oblique : fontStyleSlant
    upright : fontStyleSlant
end

declare class grBackend extends CkiaEnumItem
end
declare class grBackend_internal extends CkiaEnumClass
    dawn : grBackend
    direct3d : grBackend
    metal : grBackend
    opengl : grBackend
    vulkan : grBackend
end

declare class grSurfaceorigin extends CkiaEnumItem
end
declare class grSurfaceorigin_internal extends CkiaEnumClass
    bottom : grSurfaceorigin
    top : grSurfaceorigin
end

declare class highcontrastconfigInvertstyle extends CkiaEnumItem
end
declare class highcontrastconfigInvertstyle_internal extends CkiaEnumClass
    invertBrightness : highcontrastconfigInvertstyle
    invertLightness : highcontrastconfigInvertstyle
    noInvert : highcontrastconfigInvertstyle
end

declare class imageCachingHint extends CkiaEnumItem
end
declare class imageCachingHint_internal extends CkiaEnumClass
    allow : imageCachingHint
    disallow : imageCachingHint
end

declare class jpegencoderAlphaoption extends CkiaEnumItem
end
declare class jpegencoderAlphaoption_internal extends CkiaEnumClass
    blendOnBlack : jpegencoderAlphaoption
    ignore : jpegencoderAlphaoption
end

declare class jpegencoderDownsample extends CkiaEnumItem
end
declare class jpegencoderDownsample_internal extends CkiaEnumClass
    downsample420 : jpegencoderDownsample
    downsample422 : jpegencoderDownsample
    downsample444 : jpegencoderDownsample
end

declare class latticeRecttype extends CkiaEnumItem
end
declare class latticeRecttype_internal extends CkiaEnumClass
    default : latticeRecttype
    fixedColor : latticeRecttype
    transparent : latticeRecttype
end

declare class lineMetricStyle extends CkiaEnumItem
end
declare class lineMetricStyle_internal extends CkiaEnumClass
    css : lineMetricStyle
    typographic : lineMetricStyle
end

declare class mipmapMode extends CkiaEnumItem
end
declare class mipmapMode_internal extends CkiaEnumClass
    linear : mipmapMode
    nearest : mipmapMode
    none : mipmapMode
end

declare class paintStyle extends CkiaEnumItem
end
declare class paintStyle_internal extends CkiaEnumClass
    fill : paintStyle
    stroke : paintStyle
    strokeAndFill : paintStyle
end

declare class pathAddMode extends CkiaEnumItem
end
declare class pathAddMode_internal extends CkiaEnumClass
    append : pathAddMode
    extend : pathAddMode
end

declare class pathArcSize extends CkiaEnumItem
end
declare class pathArcSize_internal extends CkiaEnumClass
    large : pathArcSize
    small : pathArcSize
end

declare class pathDirection extends CkiaEnumItem
end
declare class pathDirection_internal extends CkiaEnumClass
    ccw : pathDirection
    cw : pathDirection
end

declare class pathEffect1dStyle extends CkiaEnumItem
end
declare class pathEffect1dStyle_internal extends CkiaEnumClass
    morph : pathEffect1dStyle
    rotate : pathEffect1dStyle
    translate : pathEffect1dStyle
end

declare class pathEffectTrimMode extends CkiaEnumItem
end
declare class pathEffectTrimMode_internal extends CkiaEnumClass
    inverted : pathEffectTrimMode
    normal : pathEffectTrimMode
end

declare class pathFilltype extends CkiaEnumItem
end
declare class pathFilltype_internal extends CkiaEnumClass
    evenodd : pathFilltype
    inverseEvenodd : pathFilltype
    inverseWinding : pathFilltype
    winding : pathFilltype
end

declare class pathSegmentMask extends CkiaEnumItem
end
declare class pathSegmentMask_internal extends CkiaEnumClass
    conic : pathSegmentMask
    cubic : pathSegmentMask
    line : pathSegmentMask
    quad : pathSegmentMask
end

declare class pathVerb extends CkiaEnumItem
end
declare class pathVerb_internal extends CkiaEnumClass
    close : pathVerb
    conic : pathVerb
    cubic : pathVerb
    done : pathVerb
    line : pathVerb
    move : pathVerb
    quad : pathVerb
end

declare class pathmeasureMatrixflags extends CkiaEnumItem
end
declare class pathmeasureMatrixflags_internal extends CkiaEnumClass
    getPosAndTan : pathmeasureMatrixflags
    getPosition : pathmeasureMatrixflags
    getTangent : pathmeasureMatrixflags
end

declare class pathop extends CkiaEnumItem
end
declare class pathop_internal extends CkiaEnumClass
    difference : pathop
    intersect : pathop
    reverseDifference : pathop
    union : pathop
    xor : pathop
end

declare class pixelgeometry extends CkiaEnumItem
end
declare class pixelgeometry_internal extends CkiaEnumClass
    bgrH : pixelgeometry
    bgrV : pixelgeometry
    rgbH : pixelgeometry
    rgbV : pixelgeometry
    unknown : pixelgeometry
end

declare class placeholderAlignment extends CkiaEnumItem
end
declare class placeholderAlignment_internal extends CkiaEnumClass
    aboveBaseline : placeholderAlignment
    baseline : placeholderAlignment
    belowBaseline : placeholderAlignment
    bottom : placeholderAlignment
    middle : placeholderAlignment
    top : placeholderAlignment
end

declare class pngencoderFilterflags extends CkiaEnumItem
end
declare class pngencoderFilterflags_internal extends CkiaEnumClass
    all : pngencoderFilterflags
    avg : pngencoderFilterflags
    none : pngencoderFilterflags
    paeth : pngencoderFilterflags
    sub : pngencoderFilterflags
    up : pngencoderFilterflags
    zero : pngencoderFilterflags
end

declare class pointMode extends CkiaEnumItem
end
declare class pointMode_internal extends CkiaEnumClass
    lines : pointMode
    points : pointMode
    polygon : pointMode
end

declare class rectHeightStyle extends CkiaEnumItem
end
declare class rectHeightStyle_internal extends CkiaEnumClass
    includeLineSpacingBottom : rectHeightStyle
    includeLineSpacingMiddle : rectHeightStyle
    includeLineSpacingTop : rectHeightStyle
    max : rectHeightStyle
    strut : rectHeightStyle
    tight : rectHeightStyle
end

declare class rectWidthStyle extends CkiaEnumItem
end
declare class rectWidthStyle_internal extends CkiaEnumClass
    max : rectWidthStyle
    tight : rectWidthStyle
end

declare class regionOp extends CkiaEnumItem
end
declare class regionOp_internal extends CkiaEnumClass
    difference : regionOp
    intersect : regionOp
    replace : regionOp
    reverseDifference : regionOp
    union : regionOp
    xor : regionOp
end

declare class rrectCorner extends CkiaEnumItem
end
declare class rrectCorner_internal extends CkiaEnumClass
    lowerLeft : rrectCorner
    lowerRight : rrectCorner
    upperLeft : rrectCorner
    upperRight : rrectCorner
end

declare class rrectType extends CkiaEnumItem
end
declare class rrectType_internal extends CkiaEnumClass
    complex : rrectType
    empty : rrectType
    ninePatch : rrectType
    oval : rrectType
    rect : rrectType
    simple : rrectType
end

declare class runtimeeffectChildType extends CkiaEnumItem
end
declare class runtimeeffectChildType_internal extends CkiaEnumClass
    blender : runtimeeffectChildType
    colorFilter : runtimeeffectChildType
    shader : runtimeeffectChildType
end

declare class runtimeeffectUniformFlags extends CkiaEnumItem
end
declare class runtimeeffectUniformFlags_internal extends CkiaEnumClass
    array : runtimeeffectUniformFlags
    color : runtimeeffectUniformFlags
    fragment : runtimeeffectUniformFlags
    halfPrecision : runtimeeffectUniformFlags
    none : runtimeeffectUniformFlags
    vertex : runtimeeffectUniformFlags
end

declare class runtimeeffectUniformType extends CkiaEnumItem
end
declare class runtimeeffectUniformType_internal extends CkiaEnumClass
    float : runtimeeffectUniformType
    float2 : runtimeeffectUniformType
    float2x2 : runtimeeffectUniformType
    float3 : runtimeeffectUniformType
    float3x3 : runtimeeffectUniformType
    float4 : runtimeeffectUniformType
    float4x4 : runtimeeffectUniformType
    int : runtimeeffectUniformType
    int2 : runtimeeffectUniformType
    int3 : runtimeeffectUniformType
    int4 : runtimeeffectUniformType
end

declare class shaderTilemode extends CkiaEnumItem
end
declare class shaderTilemode_internal extends CkiaEnumClass
    clamp : shaderTilemode
    decal : shaderTilemode
    mirror : shaderTilemode
    ['repeat'] : shaderTilemode
end

declare class skottieAnimationRenderflags extends CkiaEnumItem
end
declare class skottieAnimationRenderflags_internal extends CkiaEnumClass
    disableTopLevelClipping : skottieAnimationRenderflags
    skipTopLevelIsolation : skottieAnimationRenderflags
end

declare class strokeCap extends CkiaEnumItem
end
declare class strokeCap_internal extends CkiaEnumClass
    butt : strokeCap
    round : strokeCap
    square : strokeCap
end

declare class strokeJoin extends CkiaEnumItem
end
declare class strokeJoin_internal extends CkiaEnumClass
    bevel : strokeJoin
    miter : strokeJoin
    round : strokeJoin
end

declare class styleType extends CkiaEnumItem
end
declare class styleType_internal extends CkiaEnumClass
    allAttributes : styleType
    background : styleType
    decorations : styleType
    font : styleType
    foreground : styleType
    letterSpacing : styleType
    none : styleType
    shadow : styleType
    wordSpacing : styleType
end

declare class surfacepropsFlags extends CkiaEnumItem
end
declare class surfacepropsFlags_internal extends CkiaEnumClass
    none : surfacepropsFlags
    useDeviceIndependentFonts : surfacepropsFlags
end

declare class textAlign extends CkiaEnumItem
end
declare class textAlign_internal extends CkiaEnumClass
    center : textAlign
    ['end']: textAlign
    -- end : textAlign
    justify : textAlign
    left : textAlign
    right : textAlign
    start : textAlign
end

declare class textBaseline extends CkiaEnumItem
end
declare class textBaseline_internal extends CkiaEnumClass
    alphabetic : textBaseline
    ideographic : textBaseline
end

declare class textDecoration extends CkiaEnumItem
end
declare class textDecoration_internal extends CkiaEnumClass
    lineThrough : textDecoration
    noDecoration : textDecoration
    overline : textDecoration
    underline : textDecoration
end

declare class textDecorationMode extends CkiaEnumItem
end
declare class textDecorationMode_internal extends CkiaEnumClass
    gaps : textDecorationMode
    through : textDecorationMode
end

declare class textDecorationStyle extends CkiaEnumItem
end
declare class textDecorationStyle_internal extends CkiaEnumClass
    dashed : textDecorationStyle
    dotted : textDecorationStyle
    double : textDecorationStyle
    solid : textDecorationStyle
    wavy : textDecorationStyle
end

declare class textDirection extends CkiaEnumItem
end
declare class textDirection_internal extends CkiaEnumClass
    ltr : textDirection
    rtl : textDirection
end

declare class textEncoding extends CkiaEnumItem
end
declare class textEncoding_internal extends CkiaEnumClass
    glyphId : textEncoding
    utf16 : textEncoding
    utf32 : textEncoding
    utf8 : textEncoding
end

declare class textHeightBehavior extends CkiaEnumItem
end
declare class textHeightBehavior_internal extends CkiaEnumClass
    all : textHeightBehavior
    disableAll : textHeightBehavior
    disableFirstAscent : textHeightBehavior
    disableLastDescent : textHeightBehavior
end

declare class verticesVertexMode extends CkiaEnumItem
end
declare class verticesVertexMode_internal extends CkiaEnumClass
    triangleFan : verticesVertexMode
    triangleStrip : verticesVertexMode
    triangles : verticesVertexMode
end

declare class webpencoderCompression extends CkiaEnumItem
end
declare class webpencoderCompression_internal extends CkiaEnumClass
    lossless : webpencoderCompression
    lossy : webpencoderCompression
end
declare class ckiaEnumsList
    affinity: affinity_internal
    alphatype: alphatype_internal
    bitmapAllocflags: bitmapAllocflags_internal
    blendmode: blendmode_internal
    blurstyle: blurstyle_internal
    clipop: clipop_internal
    codecResult: codecResult_internal
    codecScanlineOrder: codecScanlineOrder_internal
    codecZeroInitialized: codecZeroInitialized_internal
    codecanimationBlend: codecanimationBlend_internal
    codecanimationDisposalmethod: codecanimationDisposalmethod_internal
    colorChannel: colorChannel_internal
    colortype: colortype_internal
    encodedImageFormat: encodedImageFormat_internal
    encodedorigin: encodedorigin_internal
    filterMode: filterMode_internal
    fontEdging: fontEdging_internal
    fontHinting: fontHinting_internal
    fontStyleSlant: fontStyleSlant_internal
    grBackend: grBackend_internal
    grSurfaceorigin: grSurfaceorigin_internal
    highcontrastconfigInvertstyle: highcontrastconfigInvertstyle_internal
    imageCachingHint: imageCachingHint_internal
    jpegencoderAlphaoption: jpegencoderAlphaoption_internal
    jpegencoderDownsample: jpegencoderDownsample_internal
    latticeRecttype: latticeRecttype_internal
    lineMetricStyle: lineMetricStyle_internal
    mipmapMode: mipmapMode_internal
    paintStyle: paintStyle_internal
    pathAddMode: pathAddMode_internal
    pathArcSize: pathArcSize_internal
    pathDirection: pathDirection_internal
    pathEffect1dStyle: pathEffect1dStyle_internal
    pathEffectTrimMode: pathEffectTrimMode_internal
    pathFilltype: pathFilltype_internal
    pathSegmentMask: pathSegmentMask_internal
    pathVerb: pathVerb_internal
    pathmeasureMatrixflags: pathmeasureMatrixflags_internal
    pathop: pathop_internal
    pixelgeometry: pixelgeometry_internal
    placeholderAlignment: placeholderAlignment_internal
    pngencoderFilterflags: pngencoderFilterflags_internal
    pointMode: pointMode_internal
    rectHeightStyle: rectHeightStyle_internal
    rectWidthStyle: rectWidthStyle_internal
    regionOp: regionOp_internal
    rrectCorner: rrectCorner_internal
    rrectType: rrectType_internal
    runtimeeffectChildType: runtimeeffectChildType_internal
    runtimeeffectUniformFlags: runtimeeffectUniformFlags_internal
    runtimeeffectUniformType: runtimeeffectUniformType_internal
    shaderTilemode: shaderTilemode_internal
    skottieAnimationRenderflags: skottieAnimationRenderflags_internal
    strokeCap: strokeCap_internal
    strokeJoin: strokeJoin_internal
    styleType: styleType_internal
    surfacepropsFlags: surfacepropsFlags_internal
    textAlign: textAlign_internal
    textBaseline: textBaseline_internal
    textDecoration: textDecoration_internal
    textDecorationMode: textDecorationMode_internal
    textDecorationStyle: textDecorationStyle_internal
    textDirection: textDirection_internal
    textEncoding: textEncoding_internal
    textHeightBehavior: textHeightBehavior_internal
    verticesVertexMode: verticesVertexMode_internal
    webpencoderCompression: webpencoderCompression_internal
end

declare class Matrix
end
declare class Shader
end
declare class MaskFilter
end
declare class Paint
    function set_shader(self, shader: Shader): nil
    function set_maskfilter(self, filter: MaskFilter): nil
    antialias: boolean
    color: Color
end
export type Circle = Vector4
export type Rect = Vector4
declare class Canvas
    function draw_circle(self, circle: Circle, paint: Paint): nil
end
export type RectTable = {
    new: (x: number, y: number, width: number, height: number) -> Rect
}
export type CircleTable = {
    new: (center_x: number, center_y: number, radius: number) -> Circle
}
export type Ckia = {
    color: ColorTable,
    enums: ckiaEnumsList,
    rect: RectTable,
    circle: CircleTable,
    new_paint: () -> Paint,
    new_linear_gradient_shader: (points: Vector4, colors: {Color}, color_pos: {number}?, tile_mode: shaderTilemode, mat: Matrix?) -> Shader,
    new_blur_filter: (style: blurstyle, sigma: number) -> MaskFilter
}
declare ckia: Ckia

