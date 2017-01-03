Imports System
Imports System.Collections.Generic
Imports System.Text
Imports System.Security.Cryptography
Imports System.Windows.Forms

Public Class Form1
    Private randomBytes() As Byte
    Private randomInt32Value As Integer
    Private possibleChars As String
    Private possibleChars1 As String
    Private possibleChars2 As String
    Private possibleChars3 As String
    Private possibleChars4 As String
    Private len As Int32
    Private GetRandomInt32Value As New RandomInt32Value
    Private GetPasswordGenProfiler As New PasswordGenProfiler
    Private sData() As String
    Private rowTel, colTel As Long
    
    Public Sub New()
        ' This call is required by the designer.
        InitializeComponent()

        ' Add any initialization after the InitializeComponent() call.
        possibleChars1 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        possibleChars2 = "abcdefghijklmnopqrstuvwxyz"
        possibleChars3 = "0123456789"
        possibleChars4 = "!@#$%^&*()_+"
        LeesCSV()
    End Sub

    Private Sub btnGenerate_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles btnGenerate.Click
        len = numPasswdSize.Value
        possibleChars = ""
        If chkAZCap.Checked Then possibleChars = possibleChars & possibleChars1
        If chkAZlow.Checked Then possibleChars = possibleChars & possibleChars2
        If chk09.Checked Then possibleChars = possibleChars & possibleChars3
        If chkSPC.Checked Then possibleChars = possibleChars & possibleChars4
        Try
            Dim cpossibleChars() As Char
            cpossibleChars = possibleChars.ToCharArray()
            If cpossibleChars.Length < 1 Then
                MessageBox.Show("You must enter one or more possible characters.")
                Return
            End If
            If len < 4 Then
                MessageBox.Show(String.Format("Please choose a password length. That length must be a value between {0} and {1}. Note: values above 1,000 might take a LONG TIME to process on some computers.", 4, Int32.MaxValue))
                Return
            End If

            Dim builder As New StringBuilder()

            For i As Integer = 0 To len - 1
                Dim randInt32 As Integer = GetRandomInt32Value.GetRandomInt()
                Dim r As New Random(randInt32)

                Dim nextInt As Integer = r.[Next](cpossibleChars.Length)
                Dim c As Char = cpossibleChars(nextInt)
                builder.Append(c)
            Next
            Me.txtPassword.Text = builder.ToString()
            fill_txtSpelled(Me.txtPassword.Text)
        Catch ex As Exception
            MessageBox.Show(String.Format("An error has occurred while trying to generate random password! Technical description: {0}", ex.Message.ToString()))
        End Try

    End Sub
    Private Sub fill_txtSpelled(pswd As String)
        Dim lenTekst As Integer
        Dim chrTeller As Integer
        Dim indx As Long
        Dim pswSpelled As String
        lenTekst = pswd.Length
        pswSpelled = ""
        For chrTeller = 1 To lenTekst
            indx = Asc(Mid(pswd, chrTeller, 1)) - 1
            If chrTeller > 1 Then pswSpelled = pswSpelled & vbCrLf
            pswSpelled = pswSpelled & grdTeksten.Rows(indx).Cells(cmbTaal.SelectedIndex).Value
        Next
        Me.txtSpelled.Text = pswSpelled
    End Sub

    Private Sub btnSpell_Click(sender As Object, e As EventArgs) Handles btnSpell.Click
        fill_txtSpelled(Me.txtPassword.Text)
    End Sub

    Private Sub Label1_Click(sender As Object, e As EventArgs) Handles Label1.Click

    End Sub

    Private Sub LeesCSV()
        Dim path As String
        path = System.IO.Path.GetDirectoryName(System.Reflection.Assembly.GetExecutingAssembly().GetName().CodeBase).Replace("\bin\Debug", "")
        path = path.Replace("file:\", "")
        ' Using ioReader As New Microsoft.VisualBasic.FileIO.TextFieldParser(path & "D:\Files\Sources\Repos\PWSpeller\PWSpeller\talen.csv")
        Using ioReader As New Microsoft.VisualBasic.FileIO.TextFieldParser(path & "\talen.csv")
            ioReader.TextFieldType = FileIO.FieldType.Delimited
            ioReader.SetDelimiters(",")

            Dim TextFileTable As DataTable = Nothing
            Dim Column As DataColumn
            Dim Row As DataRow
            Dim UpperBound As Int32
            Dim ColumnCount As Int32
            Dim CurrentRow As String()
            While Not ioReader.EndOfData
                Try
                    CurrentRow = ioReader.ReadFields()
                    If Not CurrentRow Is Nothing Then
                        If TextFileTable Is Nothing Then
                            cmbTaal.Items.Clear()
                            TextFileTable = New DataTable("TextFileTable")
                            UpperBound = CurrentRow.GetUpperBound(0)
                            For ColumnCount = 0 To UpperBound
                                Column = New DataColumn()
                                Column.DataType = System.Type.GetType("System.String")
                                Column.ColumnName = "Column" & ColumnCount
                                Column.Caption = "Column" & ColumnCount
                                Column.ReadOnly = True
                                Column.Unique = False
                                cmbTaal.Items.Add(CurrentRow(ColumnCount).ToString)
                                TextFileTable.Columns.Add(Column)
                            Next
                        End If
                        Row = TextFileTable.NewRow
                        For ColumnCount = 0 To UpperBound
                            Row("Column" & ColumnCount) = CurrentRow(ColumnCount).ToString
                        Next
                        TextFileTable.Rows.Add(Row)
                    End If
                Catch ex As Microsoft.VisualBasic.FileIO.MalformedLineException
                    MsgBox("Line " & ex.Message & "is not valid and will be skipped.")
                End Try
            End While
            ioReader.Dispose()
            grdTeksten.DataSource = TextFileTable
            cmbTaal.SelectedIndex = 0
        End Using
    End Sub
End Class


Public Class PasswordGenProfiler
    Public Shared Function GetFrequencyDistributionOfChars(allowableChars As String, generatedPass As String) As Dictionary(Of Char, Integer)
        Dim distrib As New Dictionary(Of Char, Integer)()
        ' initialize all values to 0
        For Each c As Char In allowableChars
            ' If character is listed more than once, don't re-add it to our list.
            If Not distrib.ContainsKey(c) Then
                distrib.Add(c, 0)
            End If
        Next
        Dim val As Integer = 0
        For Each passChar As Char In generatedPass
            If distrib.TryGetValue(passChar, val) Then
                distrib(passChar) = System.Threading.Interlocked.Increment(val)
            End If
        Next

        Return distrib
    End Function
End Class
Public Class RandomInt32Value
    Public Function GetRandomInt() As Integer
        Dim randomBytes As Byte() = New Byte(3) {}
        Dim rng As New RNGCryptoServiceProvider()
        rng.GetBytes(randomBytes)
        Dim randomInt As Integer = BitConverter.ToInt32(randomBytes, 0)
        Return randomInt
    End Function
End Class