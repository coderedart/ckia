declare class Color
    function __concat(self, str: string): string
end
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
declare class CkiaEnumsList
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


declare class Vec2
    x: number
    y: number
	function __add(self, other: Vec2): Vec2
    function __mul(self, other: Vec2): Vec2
    function __sub(self, other: Vec2): Vec2
end
declare class Vec3 extends Vec2
    z: number
	function __add(self, other: Vec3): Vec3
    function __mul(self, other: Vec3): Vec3
    function __sub(self, other: Vec3): Vec3
end

declare vector: (x: number, y: number, z: number) -> Vec3
export type CkiaColorTable = {
    new_alpha_rgb: (alpha: number, rgb: Vec3) -> Color,
    get_alpha: (color: Color) -> number,
    get_red: (color: Color) -> number,
    get_green: (color: Color) -> number,
    get_blue: (color: Color) -> number,
    with_alpha: (color: Color, value: number) -> Color,
    with_red: (color: Color, value: number) -> Color,
    with_green: (color: Color, value: number) -> Color,
    with_blue: (color: Color, value: number) -> Color,
    to_hex: (color: Color) -> string,
    aliceBlue: Color,
    antiqueWhite: Color,
    aqua: Color,
    aquamarine: Color,
    azure: Color,
    beige: Color,
    bisque: Color,
    black: Color,
    blanchedAlmond: Color,
    blue: Color,
    blueViolet: Color,
    brown: Color,
    burlyWood: Color,
    cadetBlue: Color,
    chartreuse: Color,
    chocolate: Color,
    coral: Color,
    cornflowerBlue: Color,
    cornsilk: Color,
    crimson: Color,
    cyan: Color,
    darkBlue: Color,
    darkCyan: Color,
    darkGoldenRod: Color,
    darkGray: Color,
    darkGreen: Color,
    darkKhaki: Color,
    darkMagenta: Color,
    darkOliveGreen: Color,
    darkorange: Color,
    darkOrchid: Color,
    darkRed: Color,
    darkSalmon: Color,
    darkSeaGreen: Color,
    darkSlateBlue: Color,
    darkSlateGray: Color,
    darkTurquoise: Color,
    darkViolet: Color,
    deepPink: Color,
    deepSkyBlue: Color,
    dimGray: Color,
    dodgerBlue: Color,
    fireBrick: Color,
    floralWhite: Color,
    forestGreen: Color,
    fuchsia: Color,
    gainsboro: Color,
    ghostWhite: Color,
    gold: Color,
    goldenRod: Color,
    gray: Color,
    green: Color,
    greenYellow: Color,
    honeyDew: Color,
    hotPink: Color,
    indianRed: Color,
    indigo: Color,
    ivory: Color,
    khaki: Color,
    lavender: Color,
    lavenderBlush: Color,
    lawnGreen: Color,
    lemonChiffon: Color,
    lightBlue: Color,
    lightCoral: Color,
    lightCyan: Color,
    lightGoldenRodYellow: Color,
    lightGrey: Color,
    lightGreen: Color,
    lightPink: Color,
    lightSalmon: Color,
    lightSeaGreen: Color,
    lightSkyBlue: Color,
    lightSlateGray: Color,
    lightSteelBlue: Color,
    lightYellow: Color,
    lime: Color,
    limeGreen: Color,
    linen: Color,
    magenta: Color,
    maroon: Color,
    mediumAquaMarine: Color,
    mediumBlue: Color,
    mediumOrchid: Color,
    mediumPurple: Color,
    mediumSeaGreen: Color,
    mediumSlateBlue: Color,
    mediumSpringGreen: Color,
    mediumTurquoise: Color,
    mediumVioletRed: Color,
    midnightBlue: Color,
    mintCream: Color,
    mistyRose: Color,
    moccasin: Color,
    navajoWhite: Color,
    navy: Color,
    oldLace: Color,
    olive: Color,
    oliveDrab: Color,
    orange: Color,
    orangeRed: Color,
    orchid: Color,
    paleGoldenRod: Color,
    paleGreen: Color,
    paleTurquoise: Color,
    paleVioletRed: Color,
    papayaWhip: Color,
    peachPuff: Color,
    peru: Color,
    pink: Color,
    plum: Color,
    powderBlue: Color,
    purple: Color,
    red: Color,
    rosyBrown: Color,
    royalBlue: Color,
    saddleBrown: Color,
    salmon: Color,
    sandyBrown: Color,
    seaGreen: Color,
    seaShell: Color,
    sienna: Color,
    silver: Color,
    skyBlue: Color,
    slateBlue: Color,
    slateGray: Color,
    snow: Color,
    springGreen: Color,
    steelBlue: Color,
    tan: Color,
    teal: Color,
    thistle: Color,
    tomato: Color,
    turquoise: Color,
    violet: Color,
    wheat: Color,
    white: Color,
    whiteSmoke: Color,
    yellow: Color,
    yellowGreen: Color,
}
declare class Matrix
    translate_x: number
    translate_y: number
    scale_x: number
    scale_y: number
    skew_x: number
    skew_y: number
    perspective_0: number
    perspective_1: number
    perspective_2: number
end
declare class Shader
end
declare class MaskFilter
end
declare class ColorFilter
end
declare class ImageFilter
end
declare class Image
    width: number
    height: number
end
declare class Region
end
declare class SkiaPath
    function clone(self): SkiaPath
    function move_to(self, point: Vec2): nil
    function line_to(self, point: Vec2): nil
    function quad_to(self, first: Vec2, second: Vec2): nil
    function conic_to(self, first: Vec2, second: Vec2, weight: number): nil
    function cubic_to(self, first: Vec2, second: Vec2, third: Vec2): nil
    function rmove_to(self, point: Vec2): nil
    function rline_to(self, point: Vec2): nil
    function rquad_to(self, first: Vec2, second: Vec2): nil
    function rconic_to(self, first: Vec2, second: Vec2, weight: number): nil
    function rcubic_to(self, first: Vec2, second: Vec2, third: Vec2): nil
    function arc_to(self, radius: Vec2, x_axis_rotate: number, large_arc: pathArcSize, sweep: pathDirection, end_point: Vec2): nil
    function rarc_to(self, radius: Vec2, x_axis_rotate: number, large_arc: pathArcSize, sweep: pathDirection, end_point: Vec2): nil
    function arc_to_with_oval(self, oval: Rect, start_angle: number, sweep_angle: number, force_move_to: boolean): nil
    function arc_to_with_points(self, first: Vec2, second: Vec2, radius: number): nil
    function close(self)
    function add_rect(self, rect: Rect, dir: pathDirection): nil
    function add_rrect(self, rrect: RRect, dir: pathDirection): nil
    function add_rrect_start(self, rrect: RRect, dir: pathDirection, start: number): nil
    function add_rounded_rect(self, rect: Rect, radius: Vec2, dir: pathDirection): nil
    function add_oval(self, rect: Rect, dir: pathDirection): nil
    function add_circle(self, center: Vec2, radius: number, dir: pathDirection): nil
    function add_arc(self, rect: Rect, start_angle: number, sweep_angle: number): nil
    function get_filltype(self): pathFilltype
    function set_filltype(self, filltype: pathFilltype): nil
    function transform(self, mat: Matrix): nil
    function transform_to_dest(self, mat: Matrix, dest: SkiaPath): nil
    function add_path_offset(self, other: SkiaPath, offset: Vec2, mode: pathAddMode): nil
    function add_path_matrix(self, other: SkiaPath, mat: Matrix, mode: pathAddMode): nil
    function add_path(self, other: SkiaPath, mode: pathAddMode): nil
    function add_path_reverse(self, other: SkiaPath): nil
    function reset(self)
    function rewind(self)
    function count_points(self): number
    function count_verbs(self): number
    function get_point(self, index: number): Vec2
    function get_bounds(self): Rect
    function compute_tight_bounds(self): Rect
    function contains(self, point: Vec2): boolean
end

declare class Paint
    antialias: boolean
    color: Color
    style: paintStyle
    stroke_width: number
    stroke_miter: number
    stroke_cap: strokeCap
    stroke_join: strokeJoin
    dither: boolean
    blendmode: blendmode

    function reset(self): nil
    function clone(self): Paint
    
    function get_shader(self): Shader?
    function set_shader(self, shader: Shader?): nil

    function get_maskfilter(self): MaskFilter?
    function set_maskfilter(self, filter:MaskFilter?): nil

    function get_colorfilter(self): ColorFilter?
    function set_colorfilter(self, filter: ColorFilter?): nil

    function get_imagefilter(self): ImageFilter?
    function set_imagefilter(self, filter: ImageFilter?): nil

    function get_path_effect(self): PathEffect?
    function set_path_effect(self, path_effect: PathEffect?): nil
end
export type Rect_Impl = {
    __index: Rect_Impl,
    new: (x: number, y: number, width: number, height: number) -> Rect,
    new_from_points: (min: Vec2, max: Vec2) -> Rect,
    clone: (self: Rect) -> Rect,
    translate: (self: Rect, offset: Vec2) -> (),
}

export type Rect = typeof(setmetatable( 
    {} :: {min: Vec2,max: Vec2,},
    {} :: Rect_Impl
))
declare class Canvas
    -- colors
    function clear(self, color: Color): nil
    function discard(self): nil
    -- canvas stack
    function get_save_count(self): number
    function save_layer(self, bounds: Rect?, paint: Paint?, backdrop: ImageFilter?, init_with_previous_layer: boolean?): number
    function save(self): number
    function restore(self): nil
    function restore_to_count(self, count: number): nil
    -- matrix
    function reset_matrix(self): nil
    function translate(self, offsets: Vec2): nil
    function scale(self, scales: Vec2): nil
    function skew(self, skews: Vec2): nil
    function rotate_degrees(self, value: number): nil
    function rotate_radians(self, value: number): nil
    -- clip
    function clip_rect(self, rect: Rect, op: clipop?, antialias: boolean?): nil
    function clip_path(self, path: SkiaPath, op: clipop?, antialias: boolean?): nil
    function clip_region(self, region: Region, op: clipop?): nil
    -- draw
    function draw_color(self, color: Color, mode: blendmode): nil
    function draw_points(self, points: {Vec2}, paint: Paint): nil
    function draw_point(self, point: Vec2, paint: Paint): nil
    function draw_line(self, from: Vec2, to: Vec2, paint: Paint): nil
    function draw_paint(self, paint: Paint): nil
    function draw_rect(self, rect: Rect, paint: Paint): nil
    function draw_simple_rect(self, left_top: Vec2, right_bottom: Vec2, paint: Paint): nil
    function draw_round_rect(self, rect: Rect, corner_radius: Vec2, paint: Paint): nil
    function draw_simple_round_rect(self, left_top: Vec2, right_bottom: Vec2, corner_radius: Vec2, paint: Paint): nil
    function draw_circle(self, center: Vec2, radius: number, paint: Paint): nil
    function draw_path(self, path: SkiaPath, paint: Paint): nil
    function draw_image(self, img: Image, point: Vec2, paint: Paint): nil
    function draw_image_rect(self, img: Image, src: Rect, dst: Rect, paint: Paint): nil

end
declare class RRect

end
declare class Fontstyle
    weight: number
    width: number
    slant: fontStyleSlant
end
declare class TextStyle
    color: Color
    has_foreground: boolean
    foreground: Paint
    has_background: boolean
    background: Paint
    decoration: textDecoration
    decoration_mode: textDecorationMode
    decoration_style: textDecorationStyle
    decoration_color: Color
    decoration_thickness_multiplier: number
    font_style: Fontstyle
    shadow_number: number
    font_feature_number: number
    font_size: number
    baseline_shift: number
    height: number
    height_override: boolean
    half_leading: boolean
    letter_spacing: number
    word_spacing: number
    function clear_foreground_color(self)
    function clear_background_color(self)
end
declare class ParagraphStyle
    text_style: TextStyle
    text_direction: textDirection
    text_height_behavior: textHeightBehavior
    text_align: textAlign
    max_lines: number
    ellipsis: string
    height: number
    replace_tab_characters: boolean
    apply_rounding_hack: boolean
    hinting: boolean
    unlimited_lines: boolean
    elipsized: boolean
    effective_align: textAlign
    function turn_hinting_off(self): nil
end
declare class FontMgr
end
declare class FontCollection
    function font_fallback_enabled(self): boolean
    function enable_font_fallback(self): nil
    function disable_font_fallback(self): nil
    function set_default_font_manager(self, fontmgr: FontMgr): nil
end
declare class Paragraph
    function get_max_width(self): number
    function get_height(self): number
    function get_min_intrinsic_width(self): number
    function get_max_intrinsic_width(self): number
    function get_alphabetic_baseline(self): number
    function get_ideographic_baseline(self): number
    function get_longest_line(self): number
    function get_did_exceed_max_lines(self): boolean
    function layout(self, width: number): nil
    function paint(self, canvas: Canvas, point: Vec2): nil
end
declare class ParagraphBuilder
    function push_style(self, style: TextStyle): nil
    function pop(self): nil
    function peek_style(self): TextStyle
    function add_text(self, text: string): nil
    function reset(self): nil
    function build(self): Paragraph
end
declare class PathEffect
end

export type Ckia = {
    color: CkiaColorTable,
    enums: CkiaEnumsList,
    rect: Rect_Impl,
    new_vec2: (x: number, y: number?) -> Vec2,
    new_paint: () -> Paint,
    new_id_matrix: () -> Matrix,
    new_path: () -> SkiaPath,
    new_rrect: () -> RRect,
    new_region: () -> Region,
    new_texstyle: () -> TextStyle,
    new_fontstyle: (weight: number, width: number, z: fontStyleSlant) -> Fontstyle,
    new_paragraph_style: () -> ParagraphStyle,
    new_font_collection: () -> FontCollection,
    new_paragraph_builder: (style: ParagraphStyle, fc: FontCollection) -> ParagraphBuilder,
    new_image_from_bytes: (bytes: string) -> Image?,

    -- mask filters
    new_blur_maskfilter: (style: blurstyle, sigma: number) -> MaskFilter,
    new_table_maskfilter: (table_bytes_256: {number}) -> MaskFilter,
    new_gamma_maskfilter: (gamme: number) -> MaskFilter,
    new_clip_maskfilter: (minimum: number, maximum: number) -> MaskFilter,
    -- color filters
    new_mode_colorfilter: (color: Color, mode: blendmode) -> ColorFilter,
    new_lighting_colorfilter: (mul: Color, add: Color) -> ColorFilter,
    new_compose_colorfilter: (outer: ColorFilter, inner: ColorFilter) -> ColorFilter,
    new_color_matrix_colorfilter: (arr_atleast_20_floats: {number}) -> ColorFilter,
    new_luma_colorfilter: () -> ColorFilter,
    -- image filters
    new_arithmetic_imagefilter: (k1: number, k2: number, k3: number, k4: number, enforce_pm_color: boolean, background: ImageFilter?, crop_rect: Rect?) -> ImageFilter,
    new_blend_imagefilter: (mode: blendmode, background: ImageFilter?, foreground: ImageFilter?, crop_rect: Rect?) -> ImageFilter,
    new_blur_imagefilter: (sigma_x: number, sigma_y: number, tile_mode: shaderTilemode, input: ImageFilter?, crop_rect: Rect?) -> ImageFilter,
    new_color_filter_imagefilter: (cf: ColorFilter, input: ImageFilter?, crop_rect: Rect?) -> ImageFilter,
    compose_imagefilter: (outer: ImageFilter, inner: ImageFilter) -> ImageFilter,
    new_displacement_map_effect_imagefilter: (x_channel_selector: number, y_channel_selector: number, scale: number, displacement: ImageFilter?, color: ImageFilter?, crop_rect: Rect?) -> ImageFilter,
    new_drop_shadow_imagefilter: (dx: number, dy: number, sigma_x: number, sigma_y: number, color: Color, input: ImageFilter?, crop_rect: Rect?) -> ImageFilter,
    new_drop_shadow_only_imagefilter: (dx: number, dy: number, sigma_x: number, sigma_y: number, color: Color, input: ImageFilter?, crop_rect: Rect?) -> ImageFilter,
    -- path effects
    compose_patheffect: (first: PathEffect, second: PathEffect) -> PathEffect,
    sum_patheffect: (first: PathEffect, second: PathEffect) -> PathEffect,
    new_discrete_patheffect: (seg_length: number, deviation: number, seed_assist: number) -> PathEffect,
    new_corner_patheffect: (radius: number) -> PathEffect,
    new_1d_path_patheffect: (path: SkiaPath, advance: number, phase: number, style: pathEffect1dStyle) -> PathEffect,
    new_2d_line_patheffect: (width: number, mat: Matrix) -> PathEffect,
    new_2d_path_patheffect: (mat: Matrix, path: SkiaPath) -> PathEffect,
    new_dash_patheffect: (intervals_atleast_2_floats: {number}, phase: number) -> PathEffect,
    new_trim_patheffect: (start: number, stop: number, mode: pathEffectTrimMode) -> PathEffect,
    -- shader
    new_empty_shader: () -> Shader,
    new_color_shader: (color: Color) -> Shader,
    new_blend_shader: (mode: blendmode, dst: Shader, src: Shader) -> Shader,
    new_linear_gradient_shader: (start: Vec2, endd: Vec2, colors: {Color},  color_pos: {number}?, tile_mode: shaderTilemode, mat: Matrix?) -> Shader,
    new_radial_gradient_shader: (center: Vec2, radius: number,  colors: {Color},  color_pos: {number}?, tile_mode: shaderTilemode, mat: Matrix?) -> Shader,
    
}
declare ckia: Ckia

declare default_font_mgr: FontMgr
