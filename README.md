# scop
## tuto
  - [toto](http://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html)
## concept
 - Vertex : un array d'informations sur le contenu d'un objet (triangle, ligne, point, ...) a render (position, couleur, ...)
 - VAO : vertex array object contient des VBO et informe sur le contenu de chaque VBO
 - VBO: vertex buffer object contient des floats informant sur la position, la couleur ...
 - pipeline : toutes les etapes par lequelles doit passer le programme pour render notre frame, certaines sont a faire (comme certains shader) d'autres se font en soum soum.
 - Shader : programmes qui tournent sur le GPU https://learnopengl.com/Getting-started/Shaders
 - Vertex Shader : 
 - Fragement Shader :
## BUGS
 - compilation error ```linking with `cc` failed: exit status: 1``` - [resolution](https://stackoverflow.com/questions/28124221/error-linking-with-cc-failed-exit-code-1)

 colors ```
 position = glGetAttribLocation(shaderProgram, 'position')
color = glGetAttribLocation(shaderProgram, 'color')

triangles = np.array(triangles, dtype=np.float32)
colorAttrib = np.array(colorAttrib, dtype=np.float32)

VBO = glGenBuffers(2)
glBindBuffer(GL_ARRAY_BUFFER, VBO[0])
glBufferData(GL_ARRAY_BUFFER, triangles.nbytes, triangles, GL_STATIC_DRAW)
glBindBuffer(GL_ARRAY_BUFFER, VBO[1])
glBufferData(GL_ARRAY_BUFFER, colorAttrib.nbytes, colorAttrib, GL_STATIC_DRAW)

glBindBuffer(GL_ARRAY_BUFFER, VBO[0])
glVertexAttribPointer(position, 3, GL_FLOAT, GL_FALSE, 0, None)
glEnableVertexAttribArray(position)

glBindBuffer(GL_ARRAY_BUFFER, VBO[1])
glVertexAttribPointer(color, 3, GL_FLOAT, GL_FALSE, 0, None)
glEnableVertexAttribArray(color)```
