<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()> _
Partial Class frmStart
    Inherits System.Windows.Forms.Form

    'Form overrides dispose to clean up the component list.
    <System.Diagnostics.DebuggerNonUserCode()> _
    Protected Overrides Sub Dispose(ByVal disposing As Boolean)
        Try
            If disposing AndAlso components IsNot Nothing Then
                components.Dispose()
            End If
        Finally
            MyBase.Dispose(disposing)
        End Try
    End Sub

    'Required by the Windows Form Designer
    Private components As System.ComponentModel.IContainer

    'NOTE: The following procedure is required by the Windows Form Designer
    'It can be modified using the Windows Form Designer.  
    'Do not modify it using the code editor.
    <System.Diagnostics.DebuggerStepThrough()> _
    Private Sub InitializeComponent()
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(frmStart))
        Me.btnGenerate = New System.Windows.Forms.Button()
        Me.numPasswdSize = New System.Windows.Forms.NumericUpDown()
        Me.Label2 = New System.Windows.Forms.Label()
        Me.txtPassword = New System.Windows.Forms.TextBox()
        Me.chkAZCap = New System.Windows.Forms.CheckBox()
        Me.chkAZlow = New System.Windows.Forms.CheckBox()
        Me.chk09 = New System.Windows.Forms.CheckBox()
        Me.chkSPC = New System.Windows.Forms.CheckBox()
        Me.txtSpelled = New System.Windows.Forms.TextBox()
        Me.btnSpell = New System.Windows.Forms.Button()
        Me.grdTeksten = New System.Windows.Forms.DataGridView()
        Me.cmbTaal = New System.Windows.Forms.ComboBox()
        Me.Label1 = New System.Windows.Forms.Label()
        CType(Me.numPasswdSize, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.grdTeksten, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.SuspendLayout()
        '
        'btnGenerate
        '
        Me.btnGenerate.Location = New System.Drawing.Point(429, 72)
        Me.btnGenerate.Name = "btnGenerate"
        Me.btnGenerate.Size = New System.Drawing.Size(75, 23)
        Me.btnGenerate.TabIndex = 0
        Me.btnGenerate.Text = "Generate"
        Me.btnGenerate.UseVisualStyleBackColor = True
        '
        'numPasswdSize
        '
        Me.numPasswdSize.Location = New System.Drawing.Point(364, 74)
        Me.numPasswdSize.Maximum = New Decimal(New Integer() {20, 0, 0, 0})
        Me.numPasswdSize.Name = "numPasswdSize"
        Me.numPasswdSize.Size = New System.Drawing.Size(59, 20)
        Me.numPasswdSize.TabIndex = 2
        Me.numPasswdSize.Value = New Decimal(New Integer() {8, 0, 0, 0})
        '
        'Label2
        '
        Me.Label2.AutoSize = True
        Me.Label2.Location = New System.Drawing.Point(282, 77)
        Me.Label2.Name = "Label2"
        Me.Label2.Size = New System.Drawing.Size(76, 13)
        Me.Label2.TabIndex = 3
        Me.Label2.Text = "Password Size"
        '
        'txtPassword
        '
        Me.txtPassword.Location = New System.Drawing.Point(26, 75)
        Me.txtPassword.Name = "txtPassword"
        Me.txtPassword.Size = New System.Drawing.Size(247, 20)
        Me.txtPassword.TabIndex = 4
        '
        'chkAZCap
        '
        Me.chkAZCap.AutoSize = True
        Me.chkAZCap.Checked = True
        Me.chkAZCap.CheckState = System.Windows.Forms.CheckState.Checked
        Me.chkAZCap.Location = New System.Drawing.Point(12, 33)
        Me.chkAZCap.Name = "chkAZCap"
        Me.chkAZCap.Size = New System.Drawing.Size(43, 17)
        Me.chkAZCap.TabIndex = 5
        Me.chkAZCap.Text = "A-Z"
        Me.chkAZCap.UseVisualStyleBackColor = True
        '
        'chkAZlow
        '
        Me.chkAZlow.AutoSize = True
        Me.chkAZlow.Checked = True
        Me.chkAZlow.CheckState = System.Windows.Forms.CheckState.Checked
        Me.chkAZlow.Location = New System.Drawing.Point(61, 33)
        Me.chkAZlow.Name = "chkAZlow"
        Me.chkAZlow.Size = New System.Drawing.Size(40, 17)
        Me.chkAZlow.TabIndex = 6
        Me.chkAZlow.Text = "a-z"
        Me.chkAZlow.UseVisualStyleBackColor = True
        '
        'chk09
        '
        Me.chk09.AutoSize = True
        Me.chk09.Checked = True
        Me.chk09.CheckState = System.Windows.Forms.CheckState.Checked
        Me.chk09.Location = New System.Drawing.Point(107, 33)
        Me.chk09.Name = "chk09"
        Me.chk09.Size = New System.Drawing.Size(41, 17)
        Me.chk09.TabIndex = 7
        Me.chk09.Text = "0-9"
        Me.chk09.UseVisualStyleBackColor = True
        '
        'chkSPC
        '
        Me.chkSPC.AutoSize = True
        Me.chkSPC.Location = New System.Drawing.Point(154, 33)
        Me.chkSPC.Name = "chkSPC"
        Me.chkSPC.Size = New System.Drawing.Size(77, 17)
        Me.chkSPC.TabIndex = 8
        Me.chkSPC.Text = "!@#$%^&*()"
        Me.chkSPC.UseVisualStyleBackColor = True
        '
        'txtSpelled
        '
        Me.txtSpelled.Location = New System.Drawing.Point(26, 141)
        Me.txtSpelled.Multiline = True
        Me.txtSpelled.Name = "txtSpelled"
        Me.txtSpelled.ScrollBars = System.Windows.Forms.ScrollBars.Vertical
        Me.txtSpelled.Size = New System.Drawing.Size(488, 257)
        Me.txtSpelled.TabIndex = 9
        '
        'btnSpell
        '
        Me.btnSpell.Location = New System.Drawing.Point(429, 101)
        Me.btnSpell.Name = "btnSpell"
        Me.btnSpell.Size = New System.Drawing.Size(75, 23)
        Me.btnSpell.TabIndex = 10
        Me.btnSpell.Text = "Spell"
        Me.btnSpell.UseVisualStyleBackColor = True
        '
        'grdTeksten
        '
        Me.grdTeksten.ColumnHeadersHeightSizeMode = System.Windows.Forms.DataGridViewColumnHeadersHeightSizeMode.AutoSize
        Me.grdTeksten.Location = New System.Drawing.Point(138, 177)
        Me.grdTeksten.Name = "grdTeksten"
        Me.grdTeksten.Size = New System.Drawing.Size(240, 150)
        Me.grdTeksten.TabIndex = 11
        '
        'cmbTaal
        '
        Me.cmbTaal.FormattingEnabled = True
        Me.cmbTaal.Location = New System.Drawing.Point(364, 33)
        Me.cmbTaal.Name = "cmbTaal"
        Me.cmbTaal.Size = New System.Drawing.Size(140, 21)
        Me.cmbTaal.TabIndex = 12
        '
        'Label1
        '
        Me.Label1.AutoSize = True
        Me.Label1.Location = New System.Drawing.Point(267, 37)
        Me.Label1.Name = "Label1"
        Me.Label1.Size = New System.Drawing.Size(91, 13)
        Me.Label1.TabIndex = 13
        Me.Label1.Text = "Spelling language"
        '
        'frmStart
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.ClientSize = New System.Drawing.Size(545, 410)
        Me.Controls.Add(Me.Label1)
        Me.Controls.Add(Me.cmbTaal)
        Me.Controls.Add(Me.btnSpell)
        Me.Controls.Add(Me.txtSpelled)
        Me.Controls.Add(Me.chkSPC)
        Me.Controls.Add(Me.chk09)
        Me.Controls.Add(Me.chkAZlow)
        Me.Controls.Add(Me.chkAZCap)
        Me.Controls.Add(Me.txtPassword)
        Me.Controls.Add(Me.Label2)
        Me.Controls.Add(Me.numPasswdSize)
        Me.Controls.Add(Me.btnGenerate)
        Me.Controls.Add(Me.grdTeksten)
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.Name = "frmStart"
        Me.Text = "PWSpeller"
        CType(Me.numPasswdSize, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.grdTeksten, System.ComponentModel.ISupportInitialize).EndInit()
        Me.ResumeLayout(False)
        Me.PerformLayout()

    End Sub

    Friend WithEvents btnGenerate As Button
    Friend WithEvents numPasswdSize As NumericUpDown
    Friend WithEvents Label2 As Label
    Friend WithEvents txtPassword As TextBox
    Friend WithEvents chkAZCap As CheckBox
    Friend WithEvents chkAZlow As CheckBox
    Friend WithEvents chk09 As CheckBox
    Friend WithEvents chkSPC As CheckBox
    Friend WithEvents txtSpelled As TextBox
    Friend WithEvents btnSpell As Button
    Friend WithEvents grdTeksten As DataGridView
    Friend WithEvents cmbTaal As ComboBox
    Friend WithEvents Label1 As Label
End Class
