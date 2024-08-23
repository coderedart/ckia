# Skia Docs
This is a simple guide to using skia


Skia's documentation is split over multiple places.

1. [Skia api](https://api.skia.org/) : contains the c++ source docs. If you want docs about a particular function or struct, this could be useful.
2. [Skia Discuss](https://groups.google.com/g/skia-discuss) : Skia forum. Most of the questions that you have are probably answered here.
3. [SkiaSharp](https://learn.microsoft.com/en-us/xamarin/xamarin-forms/user-interface/graphics/skiasharp/) : It shows you how to *use* all the functionality that Skia provides. Contains example images, which help a lot when you are starting with Skia.
4. [Flutter API](https://api.flutter.dev/flutter/dart-ui/Canvas-class.html) : Flutter draws its UI using Skia, and it has *a lot* of user friendly documentation that can be understood by UI devs. In particular, this is the only resouce which actually contains the docs for text api of Skia. It also has the best docs for explaining [blend modes](https://api.flutter.dev/flutter/dart-ui/BlendMode.html) with composite image samples.
5. [React Native Skia API](https://shopify.github.io/react-native-skia/docs/canvas/overview/) : Because of its xml like api, its *very* easy to understand teh docs. The docs unlike Flutter, are short and to the point. Great for a quick overview of most of the Skia features.
6. [Fiddle](https://fiddle.skia.org/named/) : provides a sandboxed playground online, to play with Skia. Provides lots of examples to explain various concepts, and you can edit the code to run and see the changes live in browser. 
7. [React Native Playlist](https://www.youtube.com/playlist?list=PLkOyNuxGl9jxyFx9YSRvy6dumPhofM3fs) : This is an excellent channel which shows how you can use the drawing commands to create pretty looking widgets. It uses react native, but you will still learn a lot about skia if you just follow along.
8. [Skia Src](https://github.com/google/skia/tree/main/gm): Finally, you can find a *lot* of tests/examples within skia source folder under `./gm` directory. If skia can do something, there's probably a test somewhere in this directory to test that functionality.

## API
Skia basically has three different kinds of objects
1. `Canvas` => This is where you "draw" your stuff by using the `Canvas.draw_*` functions. eg: `canvas.draw_line(x1, y1, x2, y2, paint)`
2. `Paint` => This is what you use to change the parameters of a draw function. Most draw functions take a reference to `Paint` as an argument and also document which properties of paint are used . eg: the width of border, the color of border, the fill color, any effects like masking/shading etc..
3. `Managed Objects`: These are skia objects which contain skia's internal data. eg: parsed font, compiled shaders, laid out paragraphs etc..

As an example, to draw a shape, you roughly do the following steps:
1. create a canvas. You can do it with a simple bitmap surface (provided by skia for cpu side drawing) or an opengl framebuffer (or texture) based surface (create it yourself manually).
2. Then, you need to create a rectangle struct with top,left,right,bottom coordinates. (0,0) is the top left corner of the canvas.
3. you create a paint object, and set its properties like the border width, color, whether it should fill the rectangle or only draw the border etc..
4. you finally call `canvas.draw_rect(&rect, &paint)` to draw the rect on to the canvas.
5. Now, you can simply use the jpeg/png encoder (provided by skia) to save the canvas into an image file on disk, or if you are using opengl, you will just submit the canvas so that skia can send the actual opengl commands to render this canvas. 

## Draw Commands and Caching
Skia "records" all the draw commands of a canvas and only executes them later. This allows it to optimize the drawing.
eg: draw a red rect -> then draw a blue rect. skia can optimize the execution of these draw commands by just skipping the first and directly drawing the blue rect. 

But, its still extra work if you are repeatedly drawing the same commands again and again. 
eg: At top left corner, draw a red rect -> draw a blue rect. Repeat the drawing for top right, bottom right and bottom left corners. You are basically issuing (and optimizing out) the red rect 4 times.

Skia provides a "command recorder" object called `PictureRecorder`. You create a picture recorder, use it as a canvas and finally build a picture. This allows skia to optimize the commands once. Then, you can repeatedly draw this optimized "Picture" on to your main canvas repeatedly. 

OTOH, there are times where the drawing itself is expensive. `Paragraph` is a good example of expensive draw calls. It has a large minimum cost of 100+ microseconds to draw a paragraph (regardless of size). A picture might reduce it, but text in general has a high cost. 

In such cases, it would be easier to just render to a texture (or framebuffer object) and repeatedly render the texture on to your main canvas. Its more complicated and less flexible than a picture, but useful for expensive graphics like gui widgets which use blur/shadows/complex clipping/effects etc...

Ofcourse, all of this assumes that your graphics is not changing very often. If you are using animations, then you will need to try to split your "static" drawing and "dynamic" drawing, so that you can cache static parts. 

A great example for caching is a text widget. You can cache your text drawing into a picture/texture. And you can draw a cursor or effects like text selection *over* the text. If you are doing text editing, then you might want to cache text by splitting it into lines. Changing a line won't change anything in the lines above or below. At best, when user enters a new line, you just need to offset the below lines when drawing.   

